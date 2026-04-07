//! ObservableProperties — Game-facing entity properties.
//!
//! This is the bridge between HoloSim's consciousness architecture and
//! observable, renderable, interactive game properties.

/// Game-facing entity properties — what any game engine can observe.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObservableProperties {
    /// 2D position in world space
    pub position: [f32; 2],
    /// 2D velocity
    pub velocity: [f32; 2],
    /// Mass normalized 0.0-1.0
    pub mass: f32,
    /// RGB color [0.0-1.0]
    pub color: [f32; 3],
    /// Visual size (radius in world units)
    pub size: f32,
    /// Glow intensity 0.0-1.0
    pub glow: f32,
    /// Shape for visual differentiation
    pub shape: EntityShape,
    /// Current behavioral state
    pub behavior_state: BehaviorState,
    /// Intelligence 0.0-1.0
    pub intelligence: f32,
    /// Service orientation: -1.0 (STS) to +1.0 (STO)
    pub service_orientation: f32,
    /// Density level 1-8
    pub density: u8,
    /// Top 3 active archetype indices (0-21)
    pub top_archetypes: [u8; 3],
    /// Entity is active (not dormant)
    pub is_active: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EntityShape {
    #[default]
    Circle,
    Triangle,
    Square,
    Diamond,
    Hexagon,
    Star,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BehaviorState {
    #[default]
    Idle,
    Seeking,
    Fleeing,
    Socializing,
    Contemplating,
    Leading,
}

impl Default for ObservableProperties {
    fn default() -> Self {
        ObservableProperties {
            position: [0.0, 0.0],
            velocity: [0.0, 0.0],
            mass: 0.5,
            color: [0.5, 0.5, 0.5],
            size: 1.0,
            glow: 0.0,
            shape: EntityShape::Circle,
            behavior_state: BehaviorState::Idle,
            intelligence: 0.5,
            service_orientation: 0.0,
            density: 3,
            top_archetypes: [0, 0, 0],
            is_active: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_properties() {
        let props = ObservableProperties::default();
        assert_eq!(props.position, [0.0, 0.0]);
        assert_eq!(props.velocity, [0.0, 0.0]);
        assert_eq!(props.shape, EntityShape::Circle);
        assert_eq!(props.behavior_state, BehaviorState::Idle);
        assert!(props.is_active);
        assert_eq!(props.density, 3);
    }

    #[test]
    fn test_entity_shape_is_copy() {
        let a = EntityShape::Triangle;
        let b = a;
        assert_eq!(a, b); // Both valid because Copy
    }

    #[test]
    fn test_behavior_state_is_copy() {
        let a = BehaviorState::Seeking;
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn test_observable_properties_is_copy() {
        let a = ObservableProperties::default();
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn test_custom_properties() {
        let props = ObservableProperties {
            position: [5.0, -3.0],
            color: [1.0, 0.0, 0.0],
            size: 2.0,
            shape: EntityShape::Star,
            behavior_state: BehaviorState::Leading,
            ..Default::default()
        };
        assert_eq!(props.position, [5.0, -3.0]);
        assert_eq!(props.color, [1.0, 0.0, 0.0]);
        assert_eq!(props.size, 2.0);
        assert_eq!(props.shape, EntityShape::Star);
        assert_eq!(props.behavior_state, BehaviorState::Leading);
    }
}
