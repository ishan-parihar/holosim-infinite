//! Teleological Pull - Spiritual Gravity Toward Unity
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "Teleological Pull = Field gradient toward unity (spiritual gravity)"
//!
//! # Theory
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The teleological pull is not a force but an attraction through resonance.
//!  All consciousness is drawn toward unity with Intelligent Infinity.
//!  This is spiritual gravity - the pull of the source on all manifestation."
//!
//! # Implementation
//!
//! The teleological pull operates as a field gradient that:
//! 1. Increases in strength as entity approaches source
//! 2. Guides entity evolution toward unity
//! 3. Operates through attraction, not compulsion
//! 4. Respects Free Will at all times

use crate::types::Float;

/// Gravity Vector - the direction and magnitude of teleological pull
#[derive(Debug, Clone, Copy)]
pub struct GravityVector {
    pub direction: Float,
    pub magnitude: Float,
    pub source_proximity: Float,
}

impl GravityVector {
    pub fn new() -> Self {
        Self {
            direction: 0.0,
            magnitude: 0.0,
            source_proximity: 0.0,
        }
    }

    pub fn toward_source(source_proximity: Float, current_coherence: Float) -> Self {
        let magnitude = source_proximity * current_coherence * 0.1;
        Self {
            direction: 1.0,
            magnitude,
            source_proximity,
        }
    }

    pub fn apply(&self, entity_position: Float) -> Float {
        entity_position + self.direction * self.magnitude
    }

    pub fn is_significant(&self) -> bool {
        self.magnitude > 0.001
    }
}

impl Default for GravityVector {
    fn default() -> Self {
        Self::new()
    }
}

/// Pull Strength - the intensity of teleological attraction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PullStrength {
    None,
    Weak,
    Moderate,
    Strong,
    Overwhelming,
}

impl PullStrength {
    pub fn from_magnitude(magnitude: Float) -> Self {
        if magnitude >= 0.9 {
            PullStrength::Overwhelming
        } else if magnitude >= 0.7 {
            PullStrength::Strong
        } else if magnitude >= 0.4 {
            PullStrength::Moderate
        } else if magnitude >= 0.1 {
            PullStrength::Weak
        } else {
            PullStrength::None
        }
    }

    pub fn magnitude_factor(&self) -> Float {
        match self {
            PullStrength::None => 0.0,
            PullStrength::Weak => 0.2,
            PullStrength::Moderate => 0.5,
            PullStrength::Strong => 0.8,
            PullStrength::Overwhelming => 1.0,
        }
    }

    pub fn can_resist(&self, free_will_factor: Float) -> bool {
        let resistance_threshold = 1.0 - self.magnitude_factor();
        free_will_factor > resistance_threshold
    }

    pub fn description(&self) -> &'static str {
        match self {
            PullStrength::None => "No teleological pull detected",
            PullStrength::Weak => "Gentle pull toward unity",
            PullStrength::Moderate => "Noticeable attraction to source",
            PullStrength::Strong => "Strong pull toward unity",
            PullStrength::Overwhelming => "Overwhelming attraction to source",
        }
    }
}

/// Unity Gradient - the field gradient toward unity
#[derive(Debug, Clone)]
pub struct UnityGradient {
    pub entity_id: u64,
    pub current_unity: Float,
    pub target_unity: Float,
    pub gradient_strength: Float,
    pub resistance_factor: Float,
    pub acceleration: Float,
    pub velocity: Float,
}

impl UnityGradient {
    pub fn new(entity_id: u64) -> Self {
        Self {
            entity_id,
            current_unity: 0.0,
            target_unity: 1.0,
            gradient_strength: 0.1,
            resistance_factor: 0.0,
            acceleration: 0.0,
            velocity: 0.0,
        }
    }

    pub fn with_current_unity(mut self, unity: Float) -> Self {
        self.current_unity = unity.clamp(0.0, 1.0);
        self
    }

    pub fn with_resistance(mut self, resistance: Float) -> Self {
        self.resistance_factor = resistance.clamp(0.0, 1.0);
        self
    }

    pub fn calculate_gradient(&self) -> Float {
        let distance_to_target = self.target_unity - self.current_unity;
        let base_gradient = distance_to_target * self.gradient_strength;
        let resisted_gradient = base_gradient * (1.0 - self.resistance_factor);
        resisted_gradient.max(0.0)
    }

    pub fn update(&mut self, dt: Float) {
        let gradient = self.calculate_gradient();

        self.acceleration = gradient - self.velocity * 0.1;
        self.velocity += self.acceleration * dt;

        let movement = self.velocity * dt * (1.0 - self.resistance_factor);
        self.current_unity = (self.current_unity + movement).min(self.target_unity);
    }

    pub fn apply_free_will(&mut self, free_will_factor: Float) {
        if free_will_factor > 0.5 {
            self.resistance_factor *= 0.5;
            self.velocity *= 1.1;
        } else {
            self.resistance_factor *= 1.5;
            self.velocity *= 0.9;
        }
    }

    pub fn distance_to_unity(&self) -> Float {
        self.target_unity - self.current_unity
    }

    pub fn is_near_unity(&self) -> bool {
        self.distance_to_unity() < 0.1
    }

    pub fn is_at_unity(&self) -> bool {
        self.distance_to_unity() < 0.001
    }
}

/// Unity Pull Result - the result of applying teleological pull
#[derive(Debug, Clone)]
pub struct UnityPullResult {
    pub entity_id: u64,
    pub previous_unity: Float,
    pub new_unity: Float,
    pub pull_strength: PullStrength,
    pub gradient_applied: Float,
    pub free_will_exercised: Float,
    pub resistance_overcome: Float,
    pub source_contact: bool,
}

impl UnityPullResult {
    pub fn new(
        entity_id: u64,
        previous_unity: Float,
        new_unity: Float,
        pull_strength: PullStrength,
    ) -> Self {
        Self {
            entity_id,
            previous_unity,
            new_unity,
            pull_strength,
            gradient_applied: new_unity - previous_unity,
            free_will_exercised: 0.0,
            resistance_overcome: 0.0,
            source_contact: new_unity >= 0.95,
        }
    }

    pub fn with_free_will(mut self, factor: Float) -> Self {
        self.free_will_exercised = factor;
        self
    }

    pub fn with_resistance_overcome(mut self, resistance: Float) -> Self {
        self.resistance_overcome = resistance;
        self
    }

    pub fn unity_gained(&self) -> Float {
        self.new_unity - self.previous_unity
    }

    pub fn was_progress(&self) -> bool {
        self.unity_gained() > 0.0
    }

    pub fn was_significant_progress(&self) -> bool {
        self.unity_gained() > 0.01
    }
}

/// Teleological Pull - the main teleological pull system
#[derive(Debug, Clone)]
pub struct TeleologicalPull {
    pub entity_id: u64,
    pub gravity_vector: GravityVector,
    pub unity_gradient: UnityGradient,
    pub pull_strength: PullStrength,
    pub cumulative_pull: Float,
    pub pull_history: Vec<(Float, PullStrength)>,
    pub source_contact_count: u32,
    pub last_source_contact: Float,
}

impl TeleologicalPull {
    pub fn new(entity_id: u64) -> Self {
        Self {
            entity_id,
            gravity_vector: GravityVector::new(),
            unity_gradient: UnityGradient::new(entity_id),
            pull_strength: PullStrength::None,
            cumulative_pull: 0.0,
            pull_history: Vec::new(),
            source_contact_count: 0,
            last_source_contact: 0.0,
        }
    }

    pub fn with_unity(mut self, unity: Float) -> Self {
        self.unity_gradient = UnityGradient::new(self.entity_id).with_current_unity(unity);
        self
    }

    pub fn with_resistance(mut self, resistance: Float) -> Self {
        self.unity_gradient = self.unity_gradient.clone().with_resistance(resistance);
        self
    }

    pub fn update(&mut self, dt: Float, current_time: Float) -> UnityPullResult {
        let previous_unity = self.unity_gradient.current_unity;

        self.gravity_vector = GravityVector::toward_source(
            self.unity_gradient.current_unity,
            self.calculate_coherence(),
        );

        self.unity_gradient.update(dt);

        self.pull_strength = PullStrength::from_magnitude(self.gravity_vector.magnitude);
        self.cumulative_pull += self.gravity_vector.magnitude * dt;

        let new_unity = self.unity_gradient.current_unity;

        if self.pull_strength != PullStrength::None {
            self.pull_history.push((current_time, self.pull_strength));
            if self.pull_history.len() > 100 {
                self.pull_history.remove(0);
            }
        }

        let result = UnityPullResult::new(
            self.entity_id,
            previous_unity,
            new_unity,
            self.pull_strength,
        );

        if result.source_contact {
            self.source_contact_count += 1;
            self.last_source_contact = current_time;
        }

        result
    }

    pub fn apply_free_will(&mut self, free_will_factor: Float) {
        self.unity_gradient.apply_free_will(free_will_factor);
    }

    pub fn calculate_coherence(&self) -> Float {
        self.unity_gradient.current_unity * 0.5 + 0.5
    }

    pub fn apply_external_pull(&mut self, external_magnitude: Float) {
        self.gravity_vector.magnitude += external_magnitude;
        self.pull_strength = PullStrength::from_magnitude(self.gravity_vector.magnitude);
    }

    pub fn current_distance_to_source(&self) -> Float {
        1.0 - self.unity_gradient.current_unity
    }

    pub fn is_approaching_source(&self) -> bool {
        self.unity_gradient.velocity > 0.0
    }

    pub fn pull_intensity_over_time(&self, duration: Float) -> Float {
        if self.pull_history.is_empty() {
            return 0.0;
        }

        let relevant_pulls: Vec<_> = self
            .pull_history
            .iter()
            .filter(|(t, _)| *t >= duration)
            .collect();

        if relevant_pulls.is_empty() {
            return 0.0;
        }

        let total: Float = relevant_pulls
            .iter()
            .map(|(_, s)| s.magnitude_factor())
            .sum();

        total / relevant_pulls.len() as Float
    }

    pub fn time_since_last_source_contact(&self, current_time: Float) -> Float {
        if self.last_source_contact == 0.0 {
            return Float::INFINITY;
        }
        current_time - self.last_source_contact
    }

    pub fn average_pull_strength(&self) -> Float {
        if self.pull_history.is_empty() {
            return 0.0;
        }

        self.pull_history
            .iter()
            .map(|(_, s)| s.magnitude_factor())
            .sum::<Float>()
            / self.pull_history.len() as Float
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gravity_vector_creation() {
        let gv = GravityVector::new();
        assert_eq!(gv.direction, 0.0);
        assert_eq!(gv.magnitude, 0.0);
    }

    #[test]
    fn test_gravity_vector_toward_source() {
        let gv = GravityVector::toward_source(0.5, 0.8);
        assert_eq!(gv.direction, 1.0);
        assert!(gv.magnitude > 0.0);
    }

    #[test]
    fn test_gravity_vector_apply() {
        let gv = GravityVector::toward_source(0.5, 0.8);
        let new_pos = gv.apply(0.5);
        assert!(new_pos > 0.5);
    }

    #[test]
    fn test_gravity_vector_significant() {
        let gv = GravityVector::toward_source(1.0, 1.0);
        assert!(gv.is_significant());
    }

    #[test]
    fn test_pull_strength_from_magnitude() {
        assert_eq!(PullStrength::from_magnitude(0.0), PullStrength::None);
        assert_eq!(PullStrength::from_magnitude(0.2), PullStrength::Weak);
        assert_eq!(PullStrength::from_magnitude(0.5), PullStrength::Moderate);
        assert_eq!(PullStrength::from_magnitude(0.8), PullStrength::Strong);
        assert_eq!(
            PullStrength::from_magnitude(0.95),
            PullStrength::Overwhelming
        );
    }

    #[test]
    fn test_pull_strength_can_resist() {
        let weak = PullStrength::Weak;
        assert!(weak.can_resist(0.9));
        assert!(!weak.can_resist(0.0));

        let strong = PullStrength::Strong;
        assert!(!strong.can_resist(0.1));
        assert!(strong.can_resist(0.9));
    }

    #[test]
    fn test_unity_gradient_creation() {
        let ug = UnityGradient::new(1);
        assert_eq!(ug.entity_id, 1);
        assert_eq!(ug.current_unity, 0.0);
    }

    #[test]
    fn test_unity_gradient_calculate() {
        let ug = UnityGradient::new(1).with_current_unity(0.5);
        let gradient = ug.calculate_gradient();
        assert!(gradient > 0.0);
    }

    #[test]
    fn test_unity_gradient_update() {
        let mut ug = UnityGradient::new(1).with_current_unity(0.5);
        ug.update(1.0);
        assert!(ug.current_unity > 0.5);
    }

    #[test]
    fn test_unity_gradient_resistance() {
        let ug = UnityGradient::new(1)
            .with_current_unity(0.5)
            .with_resistance(0.9);
        let gradient = ug.calculate_gradient();
        assert!(gradient < 0.1);
    }

    #[test]
    fn test_unity_gradient_near_unity() {
        let ug = UnityGradient::new(1).with_current_unity(0.95);
        assert!(ug.is_near_unity());
    }

    #[test]
    fn test_unity_pull_result_creation() {
        let result = UnityPullResult::new(1, 0.5, 0.6, PullStrength::Moderate);
        assert_eq!(result.entity_id, 1);
        assert!((result.unity_gained() - 0.1).abs() < 0.001);
        assert!(result.was_progress());
    }

    #[test]
    fn test_teleological_pull_creation() {
        let tp = TeleologicalPull::new(1);
        assert_eq!(tp.entity_id, 1);
        assert_eq!(tp.pull_strength, PullStrength::None);
    }

    #[test]
    fn test_teleological_pull_update() {
        let mut tp = TeleologicalPull::new(1).with_unity(0.5);
        let result = tp.update(1.0, 0.0);
        assert!(result.was_progress());
    }

    #[test]
    fn test_teleological_pull_free_will() {
        let mut tp = TeleologicalPull::new(1).with_unity(0.5);
        tp.apply_free_will(0.8);
        assert!(tp.unity_gradient.resistance_factor < 1.0);
    }

    #[test]
    fn test_teleological_pull_source_contact() {
        let mut tp = TeleologicalPull::new(1).with_unity(0.99);
        let result = tp.update(1.0, 0.0);
        assert!(result.source_contact);
        assert_eq!(tp.source_contact_count, 1);
    }

    #[test]
    fn test_teleological_pull_average() {
        let mut tp = TeleologicalPull::new(1).with_unity(0.5);
        tp.update(1.0, 1.0);
        tp.update(1.0, 2.0);
        let avg = tp.average_pull_strength();
        assert!(avg >= 0.0);
    }
}
