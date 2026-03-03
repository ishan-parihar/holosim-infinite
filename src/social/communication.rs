//! Communication - Density-Appropriate Communication Systems
//!
//! From V4 Roadmap Phase 6: "Social Emergence & Civilization Engine"

use crate::evolution_density_octave::density_octave::Density;
use crate::types::Float;

// ============================================================================
// Communication Types
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CommunicationType {
    /// 1st Density: No communication
    None,
    /// 2nd Density: Instinctual (pheromones, body language)
    Instinctual,
    /// 3rd Density: Verbal/symbolic
    Verbal,
    /// 4th Density: Telepathic
    Telepathic,
    /// 5th+ Density: Direct consciousness sharing
    ConsciousnessShare,
}

impl CommunicationType {
    pub fn for_density(density: Density) -> Self {
        match density {
            Density::First(_) => CommunicationType::None,
            Density::Second(_) => CommunicationType::Instinctual,
            Density::Third => CommunicationType::Verbal,
            Density::Fourth => CommunicationType::Telepathic,
            _ => CommunicationType::ConsciousnessShare,
        }
    }
}

// ============================================================================
// Message
// ============================================================================

#[derive(Debug, Clone)]
pub struct Message {
    pub sender: u64,
    pub receiver: u64,
    pub content: MessageContent,
    pub timestamp: Float,
    pub transmission_mode: TransmissionMode,
}

#[derive(Debug, Clone)]
pub enum MessageContent {
    /// Verbal: symbolic text
    Verbal(String),
    /// Emotional: feeling/impact
    Emotional { emotion: String, intensity: Float },
    /// Telepathic: direct thought
    Thought { content: String, clarity: Float },
    /// Consciousness: shared experience
    Experience {
        description: String,
        shared_wisdom: Float,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransmissionMode {
    Acoustic,
    Chemical,
    Visual,
    Telepathic,
    DirectConsciousness,
}

// ============================================================================
// Communication System
// ============================================================================

#[derive(Debug)]
pub struct CommunicationSystem {
    pub messages: Vec<Message>,
    pub max_messages: usize,
    pub telepathic_links: Vec<TelepathicLink>,
}

impl Default for CommunicationSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl CommunicationSystem {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            max_messages: 10000,
            telepathic_links: Vec::new(),
        }
    }

    /// Send a message between entities
    pub fn send_message(
        &mut self,
        sender: u64,
        receiver: u64,
        content: MessageContent,
        comm_type: CommunicationType,
    ) {
        let transmission_mode = match comm_type {
            CommunicationType::None => return,
            CommunicationType::Instinctual => TransmissionMode::Chemical,
            CommunicationType::Verbal => TransmissionMode::Acoustic,
            CommunicationType::Telepathic => TransmissionMode::Telepathic,
            CommunicationType::ConsciousnessShare => TransmissionMode::DirectConsciousness,
        };

        let message = Message {
            sender,
            receiver,
            content,
            timestamp: 0.0, // Would use simulation time
            transmission_mode,
        };

        self.messages.push(message);

        // Keep bounded
        if self.messages.len() > self.max_messages {
            self.messages.remove(0);
        }
    }

    /// Get messages for an entity
    pub fn get_messages(&self, entity: u64) -> Vec<&Message> {
        self.messages
            .iter()
            .filter(|m| m.sender == entity || m.receiver == entity)
            .collect()
    }
}

// ============================================================================
// Telepathic Link
// ============================================================================

#[derive(Debug, Clone)]
pub struct TelepathicLink {
    pub entity_a: u64,
    pub entity_b: u64,
    pub strength: Float,
    pub clarity: Float,
    pub established_at: Float,
}
