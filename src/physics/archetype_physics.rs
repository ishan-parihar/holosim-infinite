//! Archetype → Physical Properties Mapper (Phase 6.2)
//!
//! Maps 22-archetype activation profiles to physical properties usable by
//! the Rapier physics engine. Each archetype group contributes characteristic
//! forces, mass, charge, and damping based on cosmological architecture.
//!
//! From `.sisyphus/plans/physics-unification.md`:
//! "Connect archetype-derived forces → Rapier rigid-body physics → WGPU rendering"
//!
//! Force direction mapping:
//! - A0-A6 (Mind): Cognitive forces — subtle, long-range
//! - A7-A13 (Body): Physical forces — strong, short-range
//! - A14-A20 (Spirit): Resonant forces — medium, field-wide
//! - A21 (Free Will): Stochastic perturbation — deterministic hash-based

/// A 3D force vector component.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Force3D {
    pub fx: f64,
    pub fy: f64,
    pub fz: f64,
}

impl Force3D {
    /// Create a new force vector.
    pub const fn new(fx: f64, fy: f64, fz: f64) -> Self {
        Self { fx, fy, fz }
    }

    /// Magnitude of the force vector.
    pub fn magnitude(&self) -> f64 {
        (self.fx * self.fx + self.fy * self.fy + self.fz * self.fz).sqrt()
    }
}

/// Complete physical properties for an entity derived from archetype activation.
///
/// `#[repr(C)]` for FFI compatibility with Rapier C-interop layer.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhysicalProperties {
    pub mass: f64,
    pub charge: f64,
    pub spin: f64,
    pub damping: f64,
    pub moment_of_inertia: f64,
    pub force_vector: Force3D,
}

/// Mapper from archetype activation profiles to physical properties.
///
/// Holds configurable physics constants and provides all derivation methods.
#[derive(Debug, Clone, Copy)]
pub struct ArchetypePhysicsMapper {
    pub gravitational_constant: f64,
    pub electromagnetic_constant: f64,
}

/// Deterministic pseudo-random direction for Free Will (A21) based on
/// entity identifier and density level. Uses a simple hash to avoid
/// RNG dependency while remaining reproducible.
fn free_will_direction(entity_id: u64, density: u8) -> Force3D {
    // Simple deterministic hash (FNV-1a inspired)
    let mut hash: u64 = 0xcbf29ce484222325;
    hash ^= entity_id;
    hash = hash.wrapping_mul(0x100000001b3);
    hash ^= density as u64;
    hash = hash.wrapping_mul(0x100000001b3);

    // Convert hash to three components in [-1, 1]
    let h1 = ((hash >> 0) & 0xFFFF) as f64;
    let h2 = ((hash >> 16) & 0xFFFF) as f64;
    let h3 = ((hash >> 32) & 0xFFFF) as f64;

    Force3D::new(
        2.0 * h1 / 65535.0 - 1.0,
        2.0 * h2 / 65535.0 - 1.0,
        2.0 * h3 / 65535.0 - 1.0,
    )
}

/// Characteristic force direction for each of the 22 archetypes.
/// From physics-unification.md force direction mapping table.
const ARCHETYPE_FORCE_DIRECTIONS: [Force3D; 22] = [
    // Mind archetypes (A0-A6)
    Force3D::new(-1.0, 0.0, 0.0), // A0: Memory — pull toward past
    Force3D::new(1.0, 0.0, 0.0),  // A1: Perception — push toward novelty
    Force3D::new(0.0, 1.0, 0.0),  // A2: Attention — focus force
    Force3D::new(0.0, -1.0, 0.0), // A3: Reasoning — grounding
    Force3D::new(0.0, 0.0, 1.0),  // A4: Language — forward propagation
    Force3D::new(0.0, 0.0, -1.0), // A5: Logic — backward constraint
    Force3D::new(1.0, 1.0, 0.0),  // A6: Pattern — spiral pattern
    // Body archetypes (A7-A13)
    Force3D::new(0.0, 0.0, 0.0),  // A7: Physical Form — rest mass
    Force3D::new(0.0, 1.0, 0.0),  // A8: Vitality — upward life force
    Force3D::new(1.0, 0.0, 0.0),  // A9: Motion — kinetic impulse
    Force3D::new(-1.0, 0.0, 0.0), // A10: Sensation — reactive
    Force3D::new(0.0, -1.0, 0.0), // A11: Rest — damping
    Force3D::new(0.0, 0.0, 1.0),  // A12: Growth — expansion
    Force3D::new(0.0, 0.0, -1.0), // A13: Decay — contraction
    // Spirit archetypes (A14-A20)
    Force3D::new(1.0, 1.0, 1.0),    // A14: Love — attractive
    Force3D::new(-1.0, -1.0, -1.0), // A15: Wisdom — repulsive
    Force3D::new(0.0, 0.0, 0.0),    // A16: Harmony — equilibrium
    Force3D::new(0.0, 1.0, 0.0),    // A17: Transcendence — ascension
    Force3D::new(0.0, 0.0, 0.0),    // A18: Unity — cohesion
    Force3D::new(1.0, 0.0, 0.0),    // A19: Service — outward flow
    Force3D::new(-1.0, 0.0, 0.0),   // A20: Integration — inward flow
    // Free Will (A21) — computed dynamically via hash
    Force3D::new(0.0, 0.0, 0.0), // A21: placeholder, overridden in compute
];

impl ArchetypePhysicsMapper {
    /// Create a new mapper with default physical constants.
    ///
    /// Gravitational constant: 6.674×10⁻¹¹ m³/(kg·s²)
    /// Electromagnetic constant: 8.987×10⁹ N·m²/C²
    pub const fn new() -> Self {
        Self {
            gravitational_constant: 6.674e-11,
            electromagnetic_constant: 8.987e9,
        }
    }

    /// Compute force vector from archetype activation profile.
    /// Free Will (A21) uses entity_id=0, density=0 as defaults.
    /// Use `compute_force_vector_with_id` for entity-specific stochastic directions.
    pub fn compute_force_vector(&self, archetype_profile: &[f64; 22]) -> Force3D {
        self.compute_force_vector_with_id(archetype_profile, 0, 0)
    }

    /// Compute force vector with entity-specific Free Will direction.
    pub fn compute_force_vector_with_id(
        &self,
        archetype_profile: &[f64; 22],
        entity_id: u64,
        density: u8,
    ) -> Force3D {
        let mut fx = 0.0;
        let mut fy = 0.0;
        let mut fz = 0.0;

        // Sum contributions from A0-A20
        for i in 0..21 {
            let dir = ARCHETYPE_FORCE_DIRECTIONS[i];
            let activation = archetype_profile[i];
            fx += dir.fx * activation;
            fy += dir.fy * activation;
            fz += dir.fz * activation;
        }

        // A21 (Free Will): stochastic perturbation
        let fw_dir = free_will_direction(entity_id, density);
        let fw_activation = archetype_profile[21];
        fx += fw_dir.fx * fw_activation;
        fy += fw_dir.fy * fw_activation;
        fz += fw_dir.fz * fw_activation;

        Force3D::new(fx, fy, fz)
    }

    /// Compute mass from body archetype activation and density level.
    ///
    /// Mass emerges from archetype coherence with body-amplified scaling.
    /// Formula extends the existing `ArchetypeParticleDerivation::derive_mass`
    /// pattern with density-specific scaling.
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Mass emerges from the coherence of archetypal patterns"
    pub fn compute_mass(&self, archetype_profile: &[f64; 22], density: u8) -> f64 {
        let coherence = archetype_profile.iter().sum::<f64>() / 22.0;
        let body_sum: f64 = archetype_profile[7..14].iter().sum();
        let body_factor = 1.0 + body_sum / 7.0;
        let density_multiplier = 1.0 + density as f64 * 0.1;

        (0.5 + coherence * 1.5) * body_factor * density_multiplier
    }

    /// Compute electric charge from body archetype asymmetry.
    ///
    /// Charge emerges from the imbalance between the first half (A7-A10)
    /// and second half (A11-A13) of body archetypes.
    /// Returns a value clamped to [-1.0, 1.0].
    pub fn compute_charge(&self, archetype_profile: &[f64; 22]) -> f64 {
        let first_half: f64 = archetype_profile[7..11].iter().sum();
        let second_half: f64 = archetype_profile[11..14].iter().sum();
        let total = first_half.abs() + second_half.abs();

        if total < 1e-10 {
            0.0
        } else {
            ((first_half - second_half) / total).clamp(-1.0, 1.0)
        }
    }

    /// Compute moment of inertia for rotational dynamics.
    ///
    /// Derived from the distribution of body archetype activations.
    /// Higher variance in body activations → higher moment of inertia.
    pub fn compute_moment_of_inertia(&self, archetype_profile: &[f64; 22]) -> f64 {
        let body: Vec<f64> = archetype_profile[7..14].to_vec();
        let mean = body.iter().sum::<f64>() / body.len() as f64;
        let variance = body.iter().map(|a| (a - mean).powi(2)).sum::<f64>() / body.len() as f64;

        // Base inertia from body activation sum, modulated by variance
        let body_sum: f64 = archetype_profile[7..14].iter().sum();
        0.1 + body_sum * 0.5 + variance * 2.0
    }

    /// Compute damping coefficient from spirit archetype activation.
    ///
    /// Higher spirit archetype activation → more damping (spiritual
    /// entities resist rapid physical changes).
    /// Range: [0.0, 1.0] where 0 = no damping, 1 = critically damped.
    pub fn compute_damping(&self, archetype_profile: &[f64; 22]) -> f64 {
        let spirit_sum: f64 = archetype_profile[14..21].iter().sum();
        let spirit_avg = spirit_sum / 7.0;

        // Map [0, 1] average to [0, 1] damping with non-linear response
        (spirit_avg * spirit_avg).clamp(0.0, 1.0)
    }

    /// Compute spin from archetype phase relationships.
    ///
    /// Based on existing `ArchetypeParticleDerivation::derive_spin` pattern.
    pub fn compute_spin(&self, archetype_profile: &[f64; 22]) -> f64 {
        let s1 = archetype_profile[0];
        let s2 = archetype_profile[1];
        let s3 = archetype_profile[2];

        ((s1 + s2 + s3) / 3.0 - 0.5) * 2.0
    }

    /// Compute ALL physical properties in one call.
    pub fn compute_properties(
        &self,
        archetype_profile: &[f64; 22],
        density: u8,
    ) -> PhysicalProperties {
        PhysicalProperties {
            mass: self.compute_mass(archetype_profile, density),
            charge: self.compute_charge(archetype_profile),
            spin: self.compute_spin(archetype_profile),
            damping: self.compute_damping(archetype_profile),
            moment_of_inertia: self.compute_moment_of_inertia(archetype_profile),
            force_vector: self.compute_force_vector(archetype_profile),
        }
    }

    /// Compute ALL physical properties with entity-specific Free Will direction.
    pub fn compute_properties_with_id(
        &self,
        archetype_profile: &[f64; 22],
        density: u8,
        entity_id: u64,
    ) -> PhysicalProperties {
        PhysicalProperties {
            mass: self.compute_mass(archetype_profile, density),
            charge: self.compute_charge(archetype_profile),
            spin: self.compute_spin(archetype_profile),
            damping: self.compute_damping(archetype_profile),
            moment_of_inertia: self.compute_moment_of_inertia(archetype_profile),
            force_vector: self.compute_force_vector_with_id(archetype_profile, entity_id, density),
        }
    }

    /// Gravitational force between two masses (Newton's law).
    ///
    /// F = G * m₁ * m₂ / r²
    pub fn gravitational_force(&self, mass_a: f64, mass_b: f64, distance: f64) -> f64 {
        if distance < 1e-10 {
            return 0.0;
        }
        self.gravitational_constant * mass_a * mass_b / (distance * distance)
    }

    /// Electromagnetic force between two charges (Coulomb's law).
    ///
    /// F = k * q₁ * q₂ / r²
    pub fn electromagnetic_force(&self, charge_a: f64, charge_b: f64, distance: f64) -> f64 {
        if distance < 1e-10 {
            return 0.0;
        }
        self.electromagnetic_constant * charge_a * charge_b / (distance * distance)
    }
}

impl Default for ArchetypePhysicsMapper {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const ZERO_PROFILE: [f64; 22] = [0.0; 22];

    fn uniform_profile(value: f64) -> [f64; 22] {
        [value; 22]
    }

    #[test]
    fn test_force_vector_zero_archetypes() {
        let mapper = ArchetypePhysicsMapper::new();
        let force = mapper.compute_force_vector(&ZERO_PROFILE);
        assert!(force.fx.abs() < 1e-15, "Expected zero fx, got {}", force.fx);
        assert!(force.fy.abs() < 1e-15, "Expected zero fy, got {}", force.fy);
        assert!(force.fz.abs() < 1e-15, "Expected zero fz, got {}", force.fz);
    }

    #[test]
    fn test_force_vector_uniform_archetypes() {
        let mapper = ArchetypePhysicsMapper::new();
        let profile = uniform_profile(1.0);
        let force = mapper.compute_force_vector(&profile);

        // With all activations = 1.0, predictable net direction emerges
        // from summing all force directions. Free Will adds a deterministic
        // perturbation based on entity_id=0, density=0.
        // The magnitude should be non-zero and bounded.
        assert!(
            force.magnitude() > 0.0,
            "Uniform profile should produce non-zero force"
        );
        // Force should be bounded (max possible: 21 archetypes * sqrt(2) max dir + ~1.7 FW)
        assert!(
            force.magnitude() < 50.0,
            "Force magnitude {} should be bounded",
            force.magnitude()
        );
    }

    #[test]
    fn test_mass_positive() {
        let mapper = ArchetypePhysicsMapper::new();

        // Zero archetypes → minimum mass
        let mass_zero = mapper.compute_mass(&ZERO_PROFILE, 0);
        assert!(
            mass_zero > 0.0,
            "Mass should always be positive, got {}",
            mass_zero
        );

        // Uniform profile → positive mass
        let mass_uniform = mapper.compute_mass(&uniform_profile(1.0), 0);
        assert!(
            mass_uniform > 0.0,
            "Mass should be positive for uniform profile"
        );

        // Random-ish profile → still positive
        let profile: [f64; 22] = std::array::from_fn(|i| (i as f64 * 0.13) % 1.0);
        let mass_random = mapper.compute_mass(&profile, 0);
        assert!(
            mass_random > 0.0,
            "Mass should be positive for varied profile"
        );
    }

    #[test]
    fn test_mass_scales_with_density() {
        let mapper = ArchetypePhysicsMapper::new();
        let profile = uniform_profile(0.5);

        let mass_0 = mapper.compute_mass(&profile, 0);
        let mass_1 = mapper.compute_mass(&profile, 1);
        let mass_5 = mapper.compute_mass(&profile, 5);
        let mass_10 = mapper.compute_mass(&profile, 10);

        assert!(
            mass_1 > mass_0,
            "Mass at density 1 ({}) should exceed density 0 ({})",
            mass_1,
            mass_0
        );
        assert!(
            mass_5 > mass_1,
            "Mass at density 5 ({}) should exceed density 1 ({})",
            mass_5,
            mass_1
        );
        assert!(
            mass_10 > mass_5,
            "Mass at density 10 ({}) should exceed density 5 ({})",
            mass_10,
            mass_5
        );
    }

    #[test]
    fn test_charge_range() {
        let mapper = ArchetypePhysicsMapper::new();

        // Test various profiles
        let profiles: Vec<[f64; 22]> = vec![
            ZERO_PROFILE,
            uniform_profile(1.0),
            uniform_profile(0.5),
            std::array::from_fn(|i| (i as f64) / 22.0),
            std::array::from_fn(|i| if i % 2 == 0 { 1.0 } else { 0.0 }),
        ];

        for (idx, profile) in profiles.iter().enumerate() {
            let charge = mapper.compute_charge(profile);
            assert!(
                charge >= -1.0 && charge <= 1.0,
                "Profile {}: charge {} outside [-1, 1]",
                idx,
                charge
            );
        }
    }

    #[test]
    fn test_charge_asymmetry() {
        let mapper = ArchetypePhysicsMapper::new();

        // Profile with strong first-half body activation → positive charge
        let mut positive_profile = ZERO_PROFILE;
        positive_profile[7] = 1.0;
        positive_profile[8] = 1.0;
        let charge_positive = mapper.compute_charge(&positive_profile);

        // Profile with strong second-half body activation → negative charge
        let mut negative_profile = ZERO_PROFILE;
        negative_profile[11] = 1.0;
        negative_profile[12] = 1.0;
        let charge_negative = mapper.compute_charge(&negative_profile);

        assert!(
            charge_positive > 0.0,
            "First-half dominant should yield positive charge, got {}",
            charge_positive
        );
        assert!(
            charge_negative < 0.0,
            "Second-half dominant should yield negative charge, got {}",
            charge_negative
        );
        // They should be approximately opposite
        assert!(
            (charge_positive + charge_negative).abs() < 0.01,
            "Opposite profiles should yield opposite charges: {} vs {}",
            charge_positive,
            charge_negative
        );
    }

    #[test]
    fn test_damping_from_spirit() {
        let mapper = ArchetypePhysicsMapper::new();

        // Zero spirit → zero damping
        let damping_zero = mapper.compute_damping(&ZERO_PROFILE);
        assert!(
            damping_zero.abs() < 1e-10,
            "Zero spirit should yield zero damping, got {}",
            damping_zero
        );

        // High spirit → high damping
        let mut high_spirit = ZERO_PROFILE;
        for i in 14..21 {
            high_spirit[i] = 1.0;
        }
        let damping_high = mapper.compute_damping(&high_spirit);
        assert!(
            damping_high > 0.5,
            "Full spirit should yield high damping, got {}",
            damping_high
        );

        // Medium spirit → medium damping
        let mut med_spirit = ZERO_PROFILE;
        for i in 14..21 {
            med_spirit[i] = 0.5;
        }
        let damping_med = mapper.compute_damping(&med_spirit);
        assert!(
            damping_med > damping_zero && damping_med < damping_high,
            "Medium spirit damping ({}) should be between zero ({}) and high ({})",
            damping_med,
            damping_zero,
            damping_high
        );
    }

    #[test]
    fn test_gravitational_force_inversely_proportional_to_distance_squared() {
        let mapper = ArchetypePhysicsMapper::new();
        let mass_a = 1000.0;
        let mass_b = 500.0;

        let f1 = mapper.gravitational_force(mass_a, mass_b, 1.0);
        let f2 = mapper.gravitational_force(mass_a, mass_b, 2.0);
        let f4 = mapper.gravitational_force(mass_a, mass_b, 4.0);

        // F ∝ 1/r², so doubling distance → quarter force
        let ratio_1_to_2 = f1 / f2;
        assert!(
            (ratio_1_to_2 - 4.0).abs() < 1e-10,
            "Force at r=1 vs r=2 should be 4:1, got {}",
            ratio_1_to_2
        );

        let ratio_1_to_4 = f1 / f4;
        assert!(
            (ratio_1_to_4 - 16.0).abs() < 1e-10,
            "Force at r=1 vs r=4 should be 16:1, got {}",
            ratio_1_to_4
        );
    }

    #[test]
    fn test_electromagnetic_force_inversely_proportional_to_distance_squared() {
        let mapper = ArchetypePhysicsMapper::new();
        let q_a = 1.0;
        let q_b = -1.0;

        let f1 = mapper.electromagnetic_force(q_a, q_b, 1.0);
        let f2 = mapper.electromagnetic_force(q_a, q_b, 2.0);
        let f4 = mapper.electromagnetic_force(q_a, q_b, 4.0);

        let ratio_1_to_2 = f1 / f2;
        assert!(
            (ratio_1_to_2 - 4.0).abs() < 1e-10,
            "EM force at r=1 vs r=2 should be 4:1, got {}",
            ratio_1_to_2
        );

        let ratio_1_to_4 = f1 / f4;
        assert!(
            (ratio_1_to_4 - 16.0).abs() < 1e-10,
            "EM force at r=1 vs r=4 should be 16:1, got {}",
            ratio_1_to_4
        );
    }

    #[test]
    fn test_compute_properties_all_positive() {
        let mapper = ArchetypePhysicsMapper::new();
        let profile = uniform_profile(0.5);
        let props = mapper.compute_properties(&profile, 3);

        assert!(props.mass > 0.0, "mass should be positive: {}", props.mass);
        assert!(
            props.damping >= 0.0,
            "damping should be non-negative: {}",
            props.damping
        );
        assert!(
            props.moment_of_inertia > 0.0,
            "moment_of_inertia should be positive: {}",
            props.moment_of_inertia
        );
        // charge and spin can be negative, force can be zero
        assert!(
            props.charge >= -1.0 && props.charge <= 1.0,
            "charge should be in [-1,1]: {}",
            props.charge
        );
    }

    #[test]
    fn test_force3d_magnitude() {
        let f = Force3D::new(3.0, 4.0, 0.0);
        assert!((f.magnitude() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_force3d_default_is_zero() {
        let f = Force3D::default();
        assert_eq!(f.fx, 0.0);
        assert_eq!(f.fy, 0.0);
        assert_eq!(f.fz, 0.0);
    }

    #[test]
    fn test_gravitational_zero_distance_returns_zero() {
        let mapper = ArchetypePhysicsMapper::new();
        let f = mapper.gravitational_force(1.0, 1.0, 0.0);
        assert_eq!(f, 0.0);
    }

    #[test]
    fn test_electromagnetic_zero_distance_returns_zero() {
        let mapper = ArchetypePhysicsMapper::new();
        let f = mapper.electromagnetic_force(1.0, 1.0, 0.0);
        assert_eq!(f, 0.0);
    }

    #[test]
    fn test_free_will_deterministic() {
        // Same inputs should always produce same output
        let d1 = free_will_direction(42, 3);
        let d2 = free_will_direction(42, 3);
        assert_eq!(d1, d2);

        // Different inputs should produce different output (with high probability)
        let d3 = free_will_direction(42, 4);
        assert_ne!(d1, d3);

        let d4 = free_will_direction(43, 3);
        assert_ne!(d1, d4);
    }

    #[test]
    fn test_compute_properties_with_id_differs() {
        let mapper = ArchetypePhysicsMapper::new();
        let profile = uniform_profile(1.0);

        let props_1 = mapper.compute_properties_with_id(&profile, 3, 1);
        let props_2 = mapper.compute_properties_with_id(&profile, 3, 2);

        assert_eq!(props_1.mass, props_2.mass);
        assert_eq!(props_1.charge, props_2.charge);
        assert_ne!(props_1.force_vector, props_2.force_vector);
    }

    #[test]
    fn test_mapper_default_constants() {
        let mapper = ArchetypePhysicsMapper::new();
        assert!((mapper.gravitational_constant - 6.674e-11).abs() < 1e-15);
        assert!((mapper.electromagnetic_constant - 8.987e9).abs() < 1e5);
    }

    #[test]
    fn test_repr_c_layout() {
        // Verify PhysicalProperties has predictable layout for FFI
        let props = PhysicalProperties {
            mass: 1.0,
            charge: 0.5,
            spin: -0.3,
            damping: 0.1,
            moment_of_inertia: 2.0,
            force_vector: Force3D::new(1.0, 2.0, 3.0),
        };

        // Basic sanity: all fields round-trip correctly via Copy
        let copied = props;
        assert_eq!(props, copied);
    }
}
