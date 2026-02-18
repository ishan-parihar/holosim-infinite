//! Unified Module - Physics + Biology + Consciousness as One Field

pub mod unified_field;
pub mod view_factory;
pub mod physics_emergence;
pub mod biology_emergence;
pub mod consciousness_emergence;

pub use unified_field::{UnifiedField, UnifiedFieldStatistics};
pub use view_factory::{ViewFactory, PhysicsView, BiologyView, ConsciousnessView};
pub use physics_emergence::{
    PhysicsEmergence, PhysicsEmergenceConfig, PhysicsEmergenceResult,
    EmergentParticle, EmergentAtom, EmergentMolecule,
};
pub use biology_emergence::{
    BiologyEmergence, BiologyEmergenceConfig, BiologyEmergenceResult,
    EmergentDNA, LivingCell, CellType, BiologicalOrganism, BiologyPhysicsBridge,
};
pub use consciousness_emergence::{
    ConsciousnessEmergence, ConsciousnessEmergenceConfig, ConsciousnessEmergenceResult,
    Awareness, Mind, Spirit, PolarityOrientation,
    UnifiedEmergence, UnifiedEmergenceResult,
};

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    
    #[test]
    fn test_unified_field_creation() {
        let field = UnifiedField::new("test".to_string());
        assert_eq!(field.id, "test");
    }
    
    #[test]
    fn test_unified_emergence() {
        let field = Arc::new(UnifiedField::new("test".to_string()));
        let mut emergence = UnifiedEmergence::new(field);
        
        let result = emergence.process();
        assert!(result.physics.particles.len() > 0);
    }
    
    #[test]
    fn test_view_factory() {
        let field = Arc::new(UnifiedField::new("test".to_string()));
        let factory = ViewFactory::new(field);
        
        let _physics = factory.physics_view();
        let _biology = factory.biology_view();
        let _consciousness = factory.consciousness_view();
    }
}
