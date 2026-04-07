//! Veil Integration - Active Memory Filtering and Perception Shaping
//!
//! This module integrates the veil mechanics with entity consciousness,
//! actively filtering what the entity perceives and remembers.
//!
//! From V4 Roadmap Phase 5: "Archetype-Driven Consciousness Engine"
//! Integrates with existing src/veil/ module

use crate::types::Float;
use crate::veil::mechanism::{ThinSpot, VeilState};
use crate::veil::piercing::PiercingLocation;
use std::collections::VecDeque;

// ============================================================================
// Memory Filtering
// ============================================================================

/// What the veil allows through to conscious awareness
#[derive(Debug, Clone, Default)]
pub struct FilteredPerception {
    /// What the entity consciously perceives
    pub conscious_perception: ConsciousPercept,
    /// What remains in subconscious
    pub subconscious_content: SubconsciousContent,
    /// What is filtered entirely
    pub filtered_out: FilteredContent,
    /// Thin spots where veil is permeable
    pub active_thin_spots: Vec<ThinSpotInsight>,
}

/// What reaches conscious awareness through the veil
#[derive(Debug, Clone, Default)]
pub struct ConsciousPercept {
    /// What the entity sees/focuses on
    pub focus: Vec<PerceptFocus>,
    /// Emotional tone of perception
    pub emotional_tone: Float,
    /// Overall perception clarity
    pub clarity: Float,
}

/// Subconscious content that doesn't reach awareness
#[derive(Debug, Clone, Default)]
pub struct SubconsciousContent {
    /// Memories held in subconscious
    pub memories: Vec<SubconsciousMemory>,
    /// Instincts and drives
    pub instincts: Vec<Instinct>,
    /// Unprocessed catalyst
    pub pending_catalyst: Vec<CatalystBuffer>,
}

/// Content filtered entirely by the veil
#[derive(Debug, Clone, Default)]
pub struct FilteredContent {
    /// Count of filtered items
    pub count: usize,
    /// Types of content filtered
    pub types: Vec<FilteredType>,
}

#[derive(Debug, Clone, Default)]
pub enum FilteredType {
    #[default]
    Traumatic,
    KarmicPattern,
    PastLifeMemory,
    FutureInsight,
    SpiritualTruth,
    OtherSelfReality,
}

/// Insights from thin spots in the veil
#[derive(Debug, Clone)]
pub struct ThinSpotInsight {
    pub location: PiercingLocation,
    pub insight_content: String,
    pub duration: Float,
}

#[derive(Debug, Clone)]
pub struct PerceptFocus {
    pub object: String,
    pub intensity: Float,
    pub relevance: Float,
}

#[derive(Debug, Clone)]
pub struct SubconsciousMemory {
    pub memory: String,
    pub emotional_charge: Float,
    pub age: Float,
}

#[derive(Debug, Clone)]
pub struct Instinct {
    pub instinct_type: InstinctType,
    pub strength: Float,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InstinctType {
    Survival,
    Reproduction,
    Social,
    Exploration,
    Spiritual,
}

#[derive(Debug, Clone)]
pub struct CatalystBuffer {
    pub catalyst: String,
    pub intensity: Float,
    pub polarity_bias: Float,
}

// ============================================================================
// Veil Integration
// ============================================================================

/// Integrates veil mechanics with entity consciousness experience
pub struct VeilIntegration {
    /// Current veil state
    pub veil_state: VeilState,
    /// Memory buffer (what's being processed)
    memory_buffer: VecDeque<MemoryItem>,
    /// Perception history
    perception_history: VecDeque<FilteredPerception>,
    /// Thin spots and their insights
    thin_spots: Vec<ThinSpotState>,
    /// Density level affects veil
    density: u8,
    /// Polarity affects veil transparency
    #[allow(dead_code)]
    polarity: Float,
    /// Total experience accumulated
    experience_accumulated: Float,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct MemoryItem {
    content: String,
    emotional_charge: Float,
    timestamp: Float,
    processed: bool,
}

#[derive(Debug, Clone)]
struct ThinSpotState {
    location: PiercingLocation,
    strength: Float,
    insight: String,
    #[allow(dead_code)]
    last_accessed: Float,
}

impl VeilIntegration {
    /// Create veil integration for an entity at a given density
    pub fn new(density: u8) -> Self {
        // Base opacity decreases with density
        let base_opacity = match density {
            1 => 0.95, // Quantum - almost total veil
            2 => 0.85, // Molecular - heavy veil
            3 => 0.70, // Physical - standard veil
            4 => 0.40, // Emotional - thinning begins
            5 => 0.20, // Mental - significant transparency
            6 => 0.10, // Causal - thin veil
            7 => 0.00, // Spiritual - veil essentially gone
            _ => 0.00,
        };

        Self {
            veil_state: VeilState::with_opacity(base_opacity),
            memory_buffer: VecDeque::with_capacity(100),
            perception_history: VecDeque::with_capacity(50),
            thin_spots: Vec::new(),
            density,
            polarity: 0.0, // Neutral
            experience_accumulated: 0.0,
        }
    }

    /// Update veil based on entity's polarity and experience
    pub fn update(&mut self, experience_intensity: Float, growth_direction: GrowthPolarity) {
        self.experience_accumulated += experience_intensity;

        // Update polarity
        match growth_direction {
            GrowthPolarity::ServiceToOthers => {
                // STO thins the veil
                self.veil_state.opacity = (self.veil_state.opacity - 0.001).max(0.0);
            }
            GrowthPolarity::ServiceToSelf => {
                // STS thickens the veil
                self.veil_state.opacity = (self.veil_state.opacity + 0.001).min(1.0);
            }
            GrowthPolarity::Neutral => {}
        }

        // Experience accumulation can create thin spots
        if self.experience_accumulated > 10.0 && self.veil_state.opacity > 0.3 {
            self.create_thin_spot();
            self.experience_accumulated = 0.0;
        }

        // Process memory buffer
        self.process_memory_buffer();
    }

    /// Create a new thin spot in the veil
    fn create_thin_spot(&mut self) {
        // Thin spots form at significant moments
        let locations = [
            PiercingLocation::All,
            PiercingLocation::Mind,
            PiercingLocation::Spirit,
        ];

        if let Some(loc) = locations.get(self.thin_spots.len() % locations.len()) {
            let thin_spot = ThinSpot::new(*loc, 0.3, 0.0);
            self.veil_state.add_thin_spot(thin_spot);

            self.thin_spots.push(ThinSpotState {
                location: *loc,
                strength: 0.3,
                insight: "A moment of clarity emerges from within".to_string(),
                last_accessed: 0.0,
            });
        }
    }

    /// Process pending memories through the veil
    fn process_memory_buffer(&mut self) {
        // Some memories get filtered, some reach consciousness
        let permeability = 1.0 - self.veil_state.opacity;

        // Update existing thin spots
        for spot in &mut self.thin_spots {
            // Thin spots can grow with attention
            if permeability > 0.3 {
                spot.strength = (spot.strength + 0.01).min(1.0);
            }
        }

        // Keep buffer bounded
        while self.memory_buffer.len() > 50 {
            self.memory_buffer.pop_front();
        }
    }

    /// Filter incoming perception through the veil
    pub fn filter_perception(&mut self, raw_perception: RawPerception) -> FilteredPerception {
        let permeability = 1.0 - self.veil_state.opacity;

        // Determine what gets through
        let mut conscious = Vec::new();
        let mut subconscious = Vec::new();
        let mut filtered = 0;

        for percept in raw_perception.items {
            // Check against veil filter
            let passes_veil = Self::check_veil_filter(&percept, permeability, &self.thin_spots);

            if passes_veil {
                if percept.emotional_intensity * permeability > 0.3 {
                    conscious.push(PerceptFocus {
                        object: percept.content.clone(),
                        intensity: percept.emotional_intensity,
                        relevance: percept.relevance,
                    });
                } else {
                    subconscious.push(SubconsciousMemory {
                        memory: percept.content,
                        emotional_charge: percept.emotional_intensity,
                        age: 0.0,
                    });
                }
            } else {
                filtered += 1;
            }
        }

        let perception = FilteredPerception {
            conscious_perception: ConsciousPercept {
                focus: conscious,
                emotional_tone: raw_perception.emotional_tone * permeability,
                clarity: permeability,
            },
            subconscious_content: SubconsciousContent {
                memories: subconscious,
                instincts: self.get_active_instincts(),
                pending_catalyst: Vec::new(),
            },
            filtered_out: FilteredContent {
                count: filtered,
                types: vec![],
            },
            active_thin_spots: self.get_active_insights(),
        };

        // Record in history
        self.perception_history.push_back(perception.clone());
        if self.perception_history.len() > 50 {
            self.perception_history.pop_front();
        }

        perception
    }

    /// Check if content passes through the veil
    fn check_veil_filter(
        percept: &RawPerceptItem,
        permeability: Float,
        thin_spots: &[ThinSpotState],
    ) -> bool {
        // High relevance/importance content can pierce the veil
        if percept.relevance > 0.8 {
            return true;
        }

        // Emotional intensity helps content through
        if percept.emotional_intensity > 0.7 && permeability > 0.3 {
            return true;
        }

        // Check thin spots
        for spot in thin_spots {
            if spot.strength > 0.5 {
                return true;
            }
        }

        // Otherwise, random chance based on permeability
        permeability > rand::random::<Float>() * 0.5 + 0.3
    }

    /// Get currently active instincts
    fn get_active_instincts(&self) -> Vec<Instinct> {
        // Instincts are always present in subconscious
        vec![
            Instinct {
                instinct_type: InstinctType::Survival,
                strength: 1.0 - (self.density as Float / 7.0),
            },
            Instinct {
                instinct_type: InstinctType::Social,
                strength: 0.5,
            },
        ]
    }

    /// Get insights from active thin spots
    fn get_active_insights(&self) -> Vec<ThinSpotInsight> {
        self.thin_spots
            .iter()
            .filter(|s| s.strength > 0.3)
            .map(|s| ThinSpotInsight {
                location: s.location,
                insight_content: s.insight.clone(),
                duration: s.strength,
            })
            .collect()
    }

    /// Get veil transparency level
    pub fn get_transparency(&self) -> Float {
        1.0 - self.veil_state.opacity
    }

    /// Get opacity level
    pub fn get_opacity(&self) -> Float {
        self.veil_state.opacity
    }

    /// Add memory to the buffer
    pub fn add_memory(&mut self, content: String, emotional_charge: Float) {
        self.memory_buffer.push_back(MemoryItem {
            content,
            emotional_charge,
            timestamp: 0.0,
            processed: false,
        });
    }

    /// Get perception history
    pub fn get_history(&self) -> &VecDeque<FilteredPerception> {
        &self.perception_history
    }
}

/// Growth polarity
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GrowthPolarity {
    ServiceToOthers,
    ServiceToSelf,
    Neutral,
}

/// Raw perception before filtering
#[derive(Debug, Clone)]
pub struct RawPerception {
    pub items: Vec<RawPerceptItem>,
    pub emotional_tone: Float,
}

impl Default for RawPerception {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            emotional_tone: 0.5,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RawPerceptItem {
    pub content: String,
    pub emotional_intensity: Float,
    pub relevance: Float,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_veil_creation_by_density() {
        // Higher density = more transparent veil
        let veil_3 = VeilIntegration::new(3);
        let veil_6 = VeilIntegration::new(6);

        assert!(veil_6.get_opacity() < veil_3.get_opacity());
    }

    #[test]
    fn test_perception_filtering() {
        let mut veil = VeilIntegration::new(5);

        let raw = RawPerception {
            items: vec![
                RawPerceptItem {
                    content: "Important insight".to_string(),
                    emotional_intensity: 0.9,
                    relevance: 0.9,
                },
                RawPerceptItem {
                    content: "Minor detail".to_string(),
                    emotional_intensity: 0.2,
                    relevance: 0.3,
                },
            ],
            emotional_tone: 0.5,
        };

        let filtered = veil.filter_perception(raw);

        // Important content should get through
        assert!(!filtered.conscious_perception.focus.is_empty());
    }

    #[test]
    fn test_growth_affects_veil() {
        let mut veil = VeilIntegration::new(3);
        let initial_opacity = veil.get_opacity();

        // STO growth thins the veil
        veil.update(0.5, GrowthPolarity::ServiceToOthers);

        assert!(veil.get_opacity() <= initial_opacity);
    }

    #[test]
    fn test_thin_spot_creation() {
        let mut veil = VeilIntegration::new(3);

        // Accumulate enough experience to create thin spot
        for _ in 0..20 {
            veil.update(1.0, GrowthPolarity::Neutral);
        }

        assert!(!veil.thin_spots.is_empty());
    }
}
