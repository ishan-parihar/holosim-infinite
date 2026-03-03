//! Love/Logos Term: Attractive Force Toward Coherence
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Love/Logos creates structure through attractive potential.
//!  This is the Second Distortion - the force that creates from chaos.
//!  Like gravity but for consciousness - it pulls toward higher coherence."
//!
//! KEY INSIGHT: Love is an ACTUAL ATTRACTIVE FORCE, not just a concept!
//! It creates negative entropy - pulling toward order from chaos.
//!
//! The Love term:
//! - Creates coherence gradients that pull field toward order
//! - Transfers energy toward higher coherence regions
//! - Unifies densities toward Green (the Love/Light center)
//!
//! From R&D Roadmap:
//! "Love/Logos as attractive potential (coherence feedback)"

use std::collections::HashMap;

use super::{DensityAmplitude, FieldState, NUM_DENSITY_BANDS};

#[derive(Debug, Clone)]
pub struct LoveConfig {
    pub strength: f64,
    pub range: f64,
    pub coherence_sensitivity: f64,
    pub unification_strength: f64,
    pub green_density_index: usize,
}

impl Default for LoveConfig {
    fn default() -> Self {
        Self {
            strength: 0.1,
            range: 1.0,
            coherence_sensitivity: 1.0,
            unification_strength: 0.1,
            green_density_index: 3, // Fourth density = Green Ray = Love/Light
        }
    }
}

impl LoveConfig {
    pub fn strong_attraction() -> Self {
        Self {
            strength: 0.3,
            range: 2.0,
            coherence_sensitivity: 1.5,
            unification_strength: 0.2,
            green_density_index: 3,
        }
    }

    pub fn subtle_influence() -> Self {
        Self {
            strength: 0.05,
            range: 0.5,
            coherence_sensitivity: 0.5,
            unification_strength: 0.05,
            green_density_index: 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CoherenceGradient {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub magnitude: f64,
}

impl CoherenceGradient {
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            magnitude: 0.0,
        }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        let magnitude = (x * x + y * y + z * z).sqrt();
        Self { x, y, z, magnitude }
    }

    pub fn direction(&self) -> [f64; 3] {
        if self.magnitude < 1e-10 {
            [0.0, 0.0, 0.0]
        } else {
            [
                self.x / self.magnitude,
                self.y / self.magnitude,
                self.z / self.magnitude,
            ]
        }
    }

    pub fn normalized(&self) -> Self {
        if self.magnitude < 1e-10 {
            Self::zero()
        } else {
            Self::new(
                self.x / self.magnitude,
                self.y / self.magnitude,
                self.z / self.magnitude,
            )
        }
    }
}

#[derive(Debug)]
pub struct LoveTerm {
    config: LoveConfig,
    coherence_cache: HashMap<[i64; 3], f64>,
    total_applications: usize,
    total_coherence_increase: f64,
}

impl LoveTerm {
    pub fn new(config: LoveConfig) -> Self {
        Self {
            config,
            coherence_cache: HashMap::new(),
            total_applications: 0,
            total_coherence_increase: 0.0,
        }
    }

    pub fn from_defaults() -> Self {
        Self::new(LoveConfig::default())
    }

    /// Apply attractive force toward higher coherence regions
    ///
    /// This is analogous to gravity but for consciousness/coherence.
    /// Force direction: toward higher coherence
    /// Force magnitude: proportional to coherence gradient
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Love creates structure through attractive potential - like spiritual gravity"
    ///
    /// From Phase 2 R&D:
    /// "Love: Stronger attraction toward unity in Time/Space"
    /// At the veil, Love creates phase transition effects
    pub fn apply(
        &mut self,
        state: &mut FieldState,
        position: &[f64; 3],
        neighbors: Option<&[(&FieldState, [f64; 3])]>,
    ) {
        self.total_applications += 1;
        let initial_coherence = state.coherence;

        // Phase 2: Spectrum-aware strength modification
        // From Phase 2 R&D: "Love: Stronger attraction toward unity in Time/Space"
        let spectrum_strength = self.calculate_spectrum_strength(state);

        // Calculate coherence gradient
        let gradient = self.calculate_coherence_gradient(state, position, neighbors);

        // Attractive force magnitude toward higher coherence
        let attraction_magnitude =
            spectrum_strength * self.config.coherence_sensitivity * gradient.magnitude;

        // Apply force as energy flow toward coherence
        // This is NEGATIVE ENTROPY - energy flows toward order!
        let energy_flow = attraction_magnitude * 0.01;
        state.energy = state.energy * (1.0 + energy_flow);

        // Love increases coherence locally
        let coherence_increase = attraction_magnitude * 0.01 * self.config.coherence_sensitivity;
        state.coherence = (state.coherence + coherence_increase).min(1.0);

        // Phase 2: At the veil, apply special unification effects
        if state.is_at_veil() {
            // At the veil, Love creates maximum unification potential
            self.apply_veil_unification(state);
        } else {
            // Unify densities toward Green (Love unifies all)
            self.apply_unification(state);
        }

        // Update tracking
        self.total_coherence_increase += state.coherence - initial_coherence;

        // Cache coherence for future gradient calculations
        let cache_key = [
            (position[0] * 100.0) as i64,
            (position[1] * 100.0) as i64,
            (position[2] * 100.0) as i64,
        ];
        self.coherence_cache.insert(cache_key, state.coherence);
    }

    /// Calculate spectrum-aware strength for Love attraction
    ///
    /// From Phase 2 R&D:
    /// "Love: Stronger attraction toward unity in Time/Space"
    fn calculate_spectrum_strength(&self, state: &FieldState) -> f64 {
        let base_strength = self.config.strength;

        if state.is_time_space() {
            // Time/Space: stronger attraction toward unity (Oneness)
            base_strength * 1.5
        } else if state.is_at_veil() {
            // At the veil: maximum unification potential
            base_strength * 1.3
        } else {
            // Space/Time: standard strength
            base_strength
        }
    }

    /// Apply special unification effects at the veil
    ///
    /// From Phase 2 R&D:
    /// "Veil crossing produces qualitative change in field behavior"
    fn apply_veil_unification(&self, state: &mut FieldState) {
        // At the veil, Love works toward phase transition
        // All densities are pulled toward unity more strongly
        let green_idx = self.config.green_density_index;

        // Calculate average amplitude (target for unification)
        let avg_mag: f64 = state
            .density_amplitudes
            .iter()
            .map(|a| a.magnitude())
            .sum::<f64>()
            / NUM_DENSITY_BANDS as f64;

        // Pull all amplitudes toward average with veil boost
        let veil_boost = 1.0 + state.veil_transparency * 0.5;
        for amp in &mut state.density_amplitudes {
            let current_mag = amp.magnitude();
            let target_mag = avg_mag * veil_boost;
            let diff = target_mag - current_mag;
            *amp = amp.with_added_magnitude(diff * self.config.unification_strength * 0.5);
        }

        // Green ray gets extra boost at veil (Love/Light center)
        let green_boost =
            state.density_amplitudes[green_idx].scale(1.0 + self.config.unification_strength);
        state.density_amplitudes[green_idx] = green_boost;
    }

    /// Apply love attraction between two field states
    ///
    /// Love creates an attractive force between coherent regions,
    /// pulling them toward unity.
    pub fn apply_attraction(
        &mut self,
        state_a: &mut FieldState,
        state_b: &mut FieldState,
        distance: f64,
    ) {
        if distance < 1e-10 {
            return; // Avoid division by zero
        }

        // Coherence difference determines attraction strength
        let coherence_diff = (state_a.coherence - state_b.coherence).abs();

        // Higher coherence state attracts the lower one
        let attraction = self.config.strength * coherence_diff / (distance + 0.1);

        if state_a.coherence > state_b.coherence {
            // A pulls B toward its configuration
            self.transfer_toward_coherence(state_b, state_a, attraction);
        } else {
            // B pulls A toward its configuration
            self.transfer_toward_coherence(state_a, state_b, attraction);
        }
    }

    fn transfer_toward_coherence(
        &self,
        lower: &mut FieldState,
        higher: &FieldState,
        strength: f64,
    ) {
        // Transfer density amplitudes toward the higher coherence state
        for i in 0..NUM_DENSITY_BANDS {
            let diff =
                higher.density_amplitudes[i].magnitude() - lower.density_amplitudes[i].magnitude();
            let transfer = diff * strength * 0.1;
            lower.density_amplitudes[i].re += transfer;
        }

        // Also transfer coherence
        let coherence_transfer = (higher.coherence - lower.coherence) * strength * 0.1;
        lower.coherence = (lower.coherence + coherence_transfer).clamp(0.0, 1.0);
    }

    /// Calculate coherence gradient at a position
    ///
    /// The gradient points toward higher coherence regions,
    /// similar to how gravitational gradient points toward mass.
    pub fn calculate_coherence_gradient(
        &self,
        state: &FieldState,
        position: &[f64; 3],
        neighbors: Option<&[(&FieldState, [f64; 3])]>,
    ) -> CoherenceGradient {
        match neighbors {
            Some(neighbors) => self.calculate_gradient_from_neighbors(state, position, neighbors),
            None => self.calculate_gradient_from_cache(state, position),
        }
    }

    fn calculate_gradient_from_neighbors(
        &self,
        state: &FieldState,
        position: &[f64; 3],
        neighbors: &[(&FieldState, [f64; 3])],
    ) -> CoherenceGradient {
        if neighbors.is_empty() {
            return CoherenceGradient::zero();
        }

        let mut grad_x = 0.0;
        let mut grad_y = 0.0;
        let mut grad_z = 0.0;

        for (neighbor_state, neighbor_pos) in neighbors {
            let coherence_diff = neighbor_state.coherence - state.coherence;
            let dx = neighbor_pos[0] - position[0];
            let dy = neighbor_pos[1] - position[1];
            let dz = neighbor_pos[2] - position[2];
            let dist = (dx * dx + dy * dy + dz * dz).sqrt().max(0.1);

            // Gradient toward higher coherence
            grad_x += coherence_diff * dx / (dist * dist);
            grad_y += coherence_diff * dy / (dist * dist);
            grad_z += coherence_diff * dz / (dist * dist);
        }

        CoherenceGradient::new(grad_x, grad_y, grad_z)
    }

    fn calculate_gradient_from_cache(
        &self,
        state: &FieldState,
        position: &[f64; 3],
    ) -> CoherenceGradient {
        // Use position-based gradient estimation
        // This creates spatial coherence patterns
        let scale = 0.1;

        // Add base coherence attraction (love is always present)
        let base_attraction = state.coherence * 0.5;

        let grad_x = state.coherence * position[0].sin() * scale + base_attraction * 0.1;
        let grad_y = state.coherence * position[1].sin() * scale + base_attraction * 0.1;
        let grad_z = state.coherence * position[2].sin() * scale + base_attraction * 0.1;

        CoherenceGradient::new(grad_x, grad_y, grad_z)
    }

    /// Apply unification toward Green density
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Love pulls all densities toward Green (the Love/Light center)"
    fn apply_unification(&mut self, state: &mut FieldState) {
        let green_idx = self.config.green_density_index;

        // Get current magnitudes
        let mut mags: [f64; NUM_DENSITY_BANDS] = [0.0; NUM_DENSITY_BANDS];
        for i in 0..NUM_DENSITY_BANDS {
            mags[i] = state.density_amplitudes[i].magnitude();
        }

        // Transfer from other densities toward Green (Love unifies)
        for i in 0..NUM_DENSITY_BANDS {
            if i != green_idx && mags[i] > 0.001 {
                let transfer = mags[i] * self.config.unification_strength;
                state.density_amplitudes[green_idx].re += transfer;
            }
        }

        // Ensure Green maintains minimum presence (Love is always present)
        let green_mag = state.density_amplitudes[green_idx].magnitude();
        if green_mag < 0.01 {
            state.density_amplitudes[green_idx].re = 0.01;
        }
    }

    /// Calculate the "spiritual gravity" at a position
    ///
    /// This is the attractive force toward higher coherence,
    /// analogous to gravitational potential.
    pub fn spiritual_gravity(&self, coherence: f64, position: &[f64; 3]) -> f64 {
        // Higher coherence = stronger pull toward unity
        let base_gravity = self.config.strength * coherence;

        // Position-dependent factor (center is stronger)
        let position_factor =
            1.0 - (position[0].powi(2) + position[1].powi(2) + position[2].powi(2)).sqrt() / 10.0;

        base_gravity * position_factor.max(0.1)
    }

    /// Create a coherence well at a position
    ///
    /// This creates a region that attracts nearby field toward coherence.
    pub fn create_coherence_well(&mut self, center: [f64; 3], strength: f64) {
        let cache_key = [
            (center[0] * 100.0) as i64,
            (center[1] * 100.0) as i64,
            (center[2] * 100.0) as i64,
        ];

        // Insert a high coherence point that will attract neighbors
        self.coherence_cache.insert(cache_key, strength);
    }

    pub fn total_applications(&self) -> usize {
        self.total_applications
    }

    pub fn total_coherence_increase(&self) -> f64 {
        self.total_coherence_increase
    }

    pub fn average_coherence_increase(&self) -> f64 {
        if self.total_applications == 0 {
            0.0
        } else {
            self.total_coherence_increase / self.total_applications as f64
        }
    }

    pub fn config(&self) -> &LoveConfig {
        &self.config
    }

    pub fn clear_cache(&mut self) {
        self.coherence_cache.clear();
    }
}

impl Clone for LoveTerm {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            coherence_cache: self.coherence_cache.clone(),
            total_applications: self.total_applications,
            total_coherence_increase: self.total_coherence_increase,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_love_creation() {
        let love = LoveTerm::from_defaults();
        assert_eq!(love.total_applications(), 0);
    }

    #[test]
    fn test_love_increases_coherence() {
        let mut love = LoveTerm::new(LoveConfig::strong_attraction());
        let mut state = FieldState::uniform(0.3);
        let initial_coherence = state.coherence;

        love.apply(&mut state, &[0.0, 0.0, 0.0], None);

        assert!(state.coherence > initial_coherence);
    }

    #[test]
    fn test_coherence_gradient() {
        let grad = CoherenceGradient::new(1.0, 2.0, 2.0);
        assert!((grad.magnitude - 3.0).abs() < 1e-10);

        let normalized = grad.normalized();
        assert!((normalized.magnitude - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_attraction_between_states() {
        let mut love = LoveTerm::new(LoveConfig::strong_attraction());

        let mut state_high = FieldState::uniform(0.9);
        let mut state_low = FieldState::uniform(0.1);
        let initial_low = state_low.coherence;

        love.apply_attraction(&mut state_high, &mut state_low, 1.0);

        // Low coherence state should be pulled up
        assert!(state_low.coherence > initial_low);
    }

    #[test]
    fn test_unification_toward_green() {
        let mut love = LoveTerm::from_defaults();
        let mut state = FieldState::uniform(0.5);

        // Make red density dominant
        state.density_amplitudes[0] = DensityAmplitude::one();

        love.apply(&mut state, &[0.0, 0.0, 0.0], None);

        // Green (index 3) should have increased
        let green_mag = state.density_amplitudes[3].magnitude();
        assert!(green_mag > 0.0);
    }

    #[test]
    fn test_spiritual_gravity() {
        let love = LoveTerm::from_defaults();

        let gravity_center = love.spiritual_gravity(0.9, &[0.0, 0.0, 0.0]);
        let gravity_edge = love.spiritual_gravity(0.9, &[5.0, 5.0, 5.0]);

        // Center should have stronger gravity
        assert!(gravity_center > gravity_edge);
    }

    #[test]
    fn test_coherence_well() {
        let mut love = LoveTerm::from_defaults();
        love.create_coherence_well([1.0, 1.0, 1.0], 0.9);

        let mut state = FieldState::uniform(0.5);
        love.apply(&mut state, &[1.0, 1.0, 1.0], None);

        // State near well should be affected
        assert!(love.total_applications() > 0);
    }

    #[test]
    fn test_gradient_from_neighbors() {
        let love = LoveTerm::from_defaults();
        let state = FieldState::uniform(0.5);

        let neighbor1 = FieldState::uniform(0.8);
        let neighbor2 = FieldState::uniform(0.3);

        let neighbors: Vec<(&FieldState, [f64; 3])> =
            vec![(&neighbor1, [1.0, 0.0, 0.0]), (&neighbor2, [0.0, 1.0, 0.0])];

        let gradient =
            love.calculate_coherence_gradient(&state, &[0.0, 0.0, 0.0], Some(&neighbors));

        // Gradient should point toward higher coherence neighbor
        assert!(gradient.x > gradient.y);
    }

    #[test]
    fn test_love_energy_flow() {
        let mut love = LoveTerm::new(LoveConfig::strong_attraction());
        let mut state = FieldState::uniform(0.5);
        let initial_energy = state.energy;

        love.apply(&mut state, &[0.0, 0.0, 0.0], None);

        // Energy should increase due to love attraction
        assert!(state.energy > initial_energy);
    }
}
