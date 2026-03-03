//! Consciousness Emergence - Mind from Field

use super::biology_emergence::BiologicalOrganism;
use super::unified_field::UnifiedField;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ConsciousnessEmergenceConfig {
    pub awareness_threshold: f64,
    pub mind_threshold: f64,
    pub spirit_threshold: f64,
}

impl Default for ConsciousnessEmergenceConfig {
    fn default() -> Self {
        Self {
            awareness_threshold: 0.8,
            mind_threshold: 0.9,
            spirit_threshold: 0.95,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolarityOrientation {
    ServiceToOthers,
    ServiceToSelf,
    Neutral,
}

#[derive(Debug, Clone)]
pub struct Awareness {
    pub level: f64,
    pub capacity: f64,
    pub orientation: PolarityOrientation,
}

#[derive(Debug, Clone)]
pub struct Mind {
    pub awareness: Awareness,
    pub archetypes: [f64; 22],
    pub cognitive_capacity: f64,
}

#[derive(Debug, Clone)]
pub struct Spirit {
    pub mind: Mind,
    pub unity_experience: f64,
    pub transcendence_level: f64,
}

pub struct ConsciousnessEmergence {
    field: Arc<UnifiedField>,
    config: ConsciousnessEmergenceConfig,
    awareness: Vec<Awareness>,
    minds: Vec<Mind>,
    spirits: Vec<Spirit>,
}

impl ConsciousnessEmergence {
    pub fn new(field: Arc<UnifiedField>) -> Self {
        Self {
            field,
            config: ConsciousnessEmergenceConfig::default(),
            awareness: Vec::new(),
            minds: Vec::new(),
            spirits: Vec::new(),
        }
    }

    pub fn derive_awareness(&mut self, position: [f64; 3]) -> Option<Awareness> {
        let coherence = self.field.sample_coherence(position);

        if coherence > self.config.awareness_threshold {
            Some(Awareness {
                level: coherence,
                capacity: coherence,
                orientation: PolarityOrientation::Neutral,
            })
        } else {
            None
        }
    }

    pub fn derive_all_awareness(&mut self) -> Vec<Awareness> {
        self.awareness.clear();

        for i in 0..10 {
            let pos = [(i as f64) * 0.1, 0.0, 0.0];
            if let Some(aw) = self.derive_awareness(pos) {
                self.awareness.push(aw);
            }
        }

        self.awareness.clone()
    }

    pub fn derive_minds(&mut self) -> Vec<Mind> {
        self.minds.clear();

        for aw in &self.awareness {
            if aw.level > self.config.mind_threshold {
                let archetypes = self.field.get_archetypes([0.0, 0.0, 0.0]);

                self.minds.push(Mind {
                    awareness: aw.clone(),
                    archetypes,
                    cognitive_capacity: aw.level * aw.capacity,
                });
            }
        }

        self.minds.clone()
    }

    pub fn derive_spirits(&mut self) -> Vec<Spirit> {
        self.spirits.clear();

        for mind in &self.minds {
            if mind.cognitive_capacity > self.config.spirit_threshold {
                self.spirits.push(Spirit {
                    mind: mind.clone(),
                    unity_experience: mind.cognitive_capacity,
                    transcendence_level: mind.awareness.level,
                });
            }
        }

        self.spirits.clone()
    }

    pub fn emerge_all(&mut self) -> ConsciousnessEmergenceResult {
        let awareness = self.derive_all_awareness();
        let minds = self.derive_minds();
        let spirits = self.derive_spirits();

        ConsciousnessEmergenceResult {
            awareness,
            minds,
            spirits,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ConsciousnessEmergenceResult {
    pub awareness: Vec<Awareness>,
    pub minds: Vec<Mind>,
    pub spirits: Vec<Spirit>,
}

pub struct UnifiedEmergence {
    field: Arc<UnifiedField>,
    physics: super::physics_emergence::PhysicsEmergence,
    biology: super::biology_emergence::BiologyEmergence,
    consciousness: ConsciousnessEmergence,
}

impl UnifiedEmergence {
    pub fn new(field: Arc<UnifiedField>) -> Self {
        Self {
            field: field.clone(),
            physics: super::physics_emergence::PhysicsEmergence::new(field.clone()),
            biology: super::biology_emergence::BiologyEmergence::new(field.clone()),
            consciousness: ConsciousnessEmergence::new(field),
        }
    }

    pub fn process(&mut self) -> UnifiedEmergenceResult {
        let physics = self.physics.emerge_all();
        let biology = self.biology.emerge_all(&physics.molecules);
        let consciousness = self.consciousness.emerge_all();

        UnifiedEmergenceResult {
            physics,
            biology,
            consciousness,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct UnifiedEmergenceResult {
    pub physics: super::physics_emergence::PhysicsEmergenceResult,
    pub biology: super::biology_emergence::BiologyEmergenceResult,
    pub consciousness: ConsciousnessEmergenceResult,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consciousness_emergence() {
        let field = Arc::new(UnifiedField::new("test".to_string()));
        let consciousness = ConsciousnessEmergence::new(field);
        assert!(consciousness.awareness.is_empty());
    }

    #[test]
    fn test_unified_emergence() {
        let field = Arc::new(UnifiedField::new("test".to_string()));
        let mut emergence = UnifiedEmergence::new(field);
        let result = emergence.process();
        assert!(!result.physics.particles.is_empty());
    }
}
