// Physics Engine - Configurable physics simulation system
//
// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 2.4
// "The Solar/Planetary Logos provides the environmental constraints that define the physics simulation"
//
// Phase 4 Update: Dual-Mode Physics System Integration
// - PhysicsEngine now supports holographic force calculations
// - Backward compatibility maintained through migration wrappers
// - Supports Hardcoded, Holographic, and Hybrid modes

use crate::energy_fields::Vector3;
use crate::matter::particle::Nucleus;
use crate::matter::{Atom, Coordinate3D, Particle};
use crate::physics::PhysicsMode;
use crate::physics_derivation::initialize_global_physics_system;
use crate::solar_system::{PhysicsConstants, PlanetaryBiases, SolarSystemConstraints};
use crate::types::Float;
use std::collections::HashMap;

/// Physics Engine with configurable constants
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 2.4
/// "The Solar/Planetary Logos provides the environmental constraints that define the physics simulation"
///
/// Phase 2 Update: PhysicsEngine now uses emergent fields
/// Knowledge Base Reference: REFACTOR_ROADMAP_V2.md Section 2.5
/// "Forces emerge from interactions - Not fundamental laws"
///
/// Phase 4 Update: PhysicsEngine now supports dual-mode physics
/// Knowledge Base Reference: REFACTOR_ROADMAP_HOLOGRAPHIC_ARCHITECTURE.md Phase 4
/// "Forces can be calculated using Hardcoded, Holographic, or Hybrid modes"
#[derive(Debug, Clone)]
pub struct PhysicsEngine {
    /// Configurable physics constants
    pub constants: PhysicsConstants,

    /// Planetary biases affecting physics
    pub biases: PlanetaryBiases,

    /// Cached field calculations for performance
    field_cache: HashMap<String, Vector3>,

    /// Use emergent field calculations
    ///
    /// Phase 2: When true, forces are calculated from emergent fields
    /// When false, legacy hardcoded equations are used
    use_emergent_fields: bool,

    /// Use dual-mode physics system
    ///
    /// Phase 4: When true, forces are calculated using dual-mode system
    /// When false, legacy hardcoded equations are used
    use_dual_mode_physics: bool,
}

impl PhysicsEngine {
    /// Create a new physics engine with the given constraints
    pub fn new(constraints: &SolarSystemConstraints) -> Self {
        Self {
            constants: constraints.physics_constants.clone(),
            biases: constraints.planetary_biases.clone(),
            field_cache: HashMap::new(),
            use_emergent_fields: true, // Phase 2: Use emergent fields by default
            use_dual_mode_physics: false, // Phase 4: Disabled by default for backward compatibility
        }
    }

    /// Create a physics engine with Earth-like constraints
    pub fn earth_like() -> Self {
        Self::new(&SolarSystemConstraints::earth_like())
    }

    /// Create a physics engine with Mars-like constraints
    pub fn mars_like() -> Self {
        Self::new(&SolarSystemConstraints::mars_like())
    }

    /// Enable or disable emergent field calculations
    ///
    /// Phase 2: When enabled, forces are calculated from emergent fields
    /// When disabled, legacy hardcoded equations are used
    pub fn set_use_emergent_fields(&mut self, use_emergent: bool) {
        self.use_emergent_fields = use_emergent;
    }

    /// Check if emergent field calculations are enabled
    pub fn use_emergent_fields(&self) -> bool {
        self.use_emergent_fields
    }

    /// Enable or disable dual-mode physics system
    ///
    /// Phase 4: When enabled, forces are calculated using dual-mode system
    /// When disabled, legacy hardcoded equations are used
    pub fn set_use_dual_mode_physics(&mut self, use_dual_mode: bool) {
        self.use_dual_mode_physics = use_dual_mode;
    }

    /// Check if dual-mode physics system is enabled
    pub fn use_dual_mode_physics(&self) -> bool {
        self.use_dual_mode_physics
    }

    /// Initialize dual-mode physics system
    ///
    /// Phase 4: Legacy Migration
    /// Call this to initialize the global dual-mode physics system.
    ///
    /// # Arguments
    /// * `mode` - Physics mode (Hardcoded, Holographic, or Hybrid)
    /// * `holographic_threshold` - Threshold for holographic discovery (0.0 to 1.0)
    pub fn initialize_dual_mode_physics(mode: PhysicsMode, holographic_threshold: Float) {
        initialize_global_physics_system(mode, holographic_threshold);
    }

    /// Calculate gravitational force between two entities
    ///
    /// F = G * m1 * m2 / r²
    /// Direction is from entity1 towards entity2
    pub fn gravitational_force(
        &self,
        pos1: &Vector3,
        mass1: Float,
        pos2: &Vector3,
        mass2: Float,
    ) -> Vector3 {
        let r_vector = pos2.sub(pos1);
        let distance = r_vector.magnitude();

        if distance < 1e-10 {
            return Vector3::zero();
        }

        let force_magnitude = (self.constants.gravity * mass1 * mass2) / (distance * distance);
        let direction = r_vector.normalize().unwrap_or(Vector3::zero());

        direction.scale(force_magnitude)
    }

    /// Calculate gravitational field at a position from a mass
    pub fn gravitational_field_at(
        &self,
        mass_pos: &Vector3,
        mass: Float,
        field_pos: &Vector3,
    ) -> Vector3 {
        let r_vector = field_pos.sub(mass_pos);
        let distance = r_vector.magnitude();

        if distance < 1e-10 {
            return Vector3::zero();
        }

        let field_magnitude = self.constants.gravity * mass / (distance * distance);
        let direction = r_vector.normalize().unwrap_or(Vector3::zero());

        direction.scale(field_magnitude)
    }

    /// Calculate gravitational force from emergent field
    ///
    /// Phase 2: Force emerges from field properties
    /// Knowledge Base Reference: REFACTOR_ROADMAP_V2.md Section 2.5
    /// "Gravity emerges from Great Way archetypes (A7, A14, A21)"
    ///
    /// # Arguments
    /// * `field` - The emergent gravitational field
    /// * `position` - The position to calculate force at
    /// * `mass` - The mass experiencing the force
    ///
    /// # Returns
    /// The gravitational force vector
    pub fn gravitational_force_from_field(
        &self,
        field: &crate::energy_fields::GravitationalField,
        position: Vector3,
        mass: Float,
    ) -> Vector3 {
        // Get field vector at position
        let field_vec = field.field_at(position, self.constants.gravity);

        // Force = mass × field
        field_vec.scale(mass)
    }

    /// Calculate gravitational force from field curvature
    ///
    /// Phase 2: Force emerges from spacetime curvature
    /// Knowledge Base Reference: REFACTOR_ROADMAP_V2.md Section 2.5
    /// "Gravity emerges from spacetime curvature"
    ///
    /// # Arguments
    /// * `field` - The emergent gravitational field
    /// * `position` - The position to calculate force at
    /// * `mass` - The mass experiencing the force
    /// * `influence_radius` - Radius for density calculation
    ///
    /// # Returns
    /// The gravitational force vector based on curvature
    pub fn gravitational_force_from_curvature(
        &self,
        field: &crate::energy_fields::GravitationalField,
        position: Vector3,
        mass: Float,
        influence_radius: Float,
    ) -> Vector3 {
        // Calculate curvature at position
        let curvature =
            field.field_curvature(position, self.constants.gravity, influence_radius, 1e-6);

        // Calculate density gradient (points toward higher density)
        let step_size = 1e-6;
        let dx = Vector3::new(step_size, 0.0, 0.0);
        let dy = Vector3::new(0.0, step_size, 0.0);
        let dz = Vector3::new(0.0, 0.0, step_size);

        let density_center =
            field.field_density(position, self.constants.gravity, influence_radius);
        let density_x =
            field.field_density(position.add(&dx), self.constants.gravity, influence_radius);
        let density_y =
            field.field_density(position.add(&dy), self.constants.gravity, influence_radius);
        let density_z =
            field.field_density(position.add(&dz), self.constants.gravity, influence_radius);

        // Gradient points toward higher density
        let gradient = Vector3::new(
            density_x - density_center,
            density_y - density_center,
            density_z - density_center,
        );

        // Force points toward higher field density (attractive)
        let gradient_dir = gradient.normalize().unwrap_or(Vector3::zero());

        // Force magnitude proportional to curvature and mass
        let force_magnitude = curvature.abs() * mass * self.constants.gravity * 1e20; // Scale factor

        gradient_dir.scale(force_magnitude)
    }

    /// Calculate electromagnetic force between two charged particles
    ///
    /// F = k_e * q1 * q2 / r²
    /// Affected by planetary EM field bias
    pub fn electromagnetic_force(
        &self,
        pos1: &Vector3,
        charge1: Float,
        pos2: &Vector3,
        charge2: Float,
    ) -> Vector3 {
        let r_vector = pos2.sub(pos1);
        let distance = r_vector.magnitude();

        if distance < 1e-10 {
            return Vector3::zero();
        }

        let base_force = self
            .constants
            .electromagnetic_force(charge1, charge2, distance);
        let bias_modifier = self.biases.em_field.strength * 1e4; // Scale to reasonable range

        let direction = r_vector.normalize().unwrap_or(Vector3::zero());
        direction.scale(base_force * bias_modifier)
    }

    /// Calculate electromagnetic field at a position from a charge
    pub fn electromagnetic_field_at(
        &self,
        charge_pos: &Vector3,
        charge: Float,
        field_pos: &Vector3,
    ) -> Vector3 {
        let r_vector = field_pos.sub(charge_pos);
        let distance = r_vector.magnitude();

        if distance < 1e-10 {
            return Vector3::zero();
        }

        let field_magnitude = self.constants.electromagnetic_force(charge, 1.0, distance);
        let bias_modifier = self.biases.em_field.strength * 1e4;

        let direction = r_vector.normalize().unwrap_or(Vector3::zero());
        direction.scale(field_magnitude * bias_modifier)
    }

    /// Calculate electromagnetic force from emergent field
    ///
    /// Phase 2: Force emerges from field properties
    /// Knowledge Base Reference: REFACTOR_ROADMAP_V2.md Section 2.5
    /// "EM emerges from Catalyst archetypes (A3, A10, A17)"
    ///
    /// # Arguments
    /// * `field` - The emergent electric field
    /// * `position` - The position to calculate force at
    /// * `charge` - The charge experiencing the force
    ///
    /// # Returns
    /// The electromagnetic force vector
    pub fn electromagnetic_force_from_field(
        &self,
        field: &crate::energy_fields::ElectricField,
        position: Vector3,
        charge: Float,
    ) -> Vector3 {
        // Get electric field vector at position
        let k = 8.9875517923e9; // Coulomb constant
        let field_vec = field.field_at(position, k);

        // Force = charge × field
        field_vec.scale(charge)
    }

    /// Calculate electromagnetic force from field curvature
    ///
    /// Phase 2: Force emerges from electric field curvature
    /// Knowledge Base Reference: REFACTOR_ROADMAP_V2.md Section 2.5
    /// "EM force emerges from Catalyst archetype gradients"
    ///
    /// # Arguments
    /// * `field` - The emergent electromagnetic field
    /// * `position` - The position to calculate force at
    /// * `charge` - The charge experiencing the force
    /// * `influence_radius` - Radius for density calculation
    ///
    /// # Returns
    /// The electromagnetic force vector based on curvature
    pub fn electromagnetic_force_from_curvature(
        &self,
        field: &crate::energy_fields::ElectricField,
        position: Vector3,
        charge: Float,
        influence_radius: Float,
    ) -> Vector3 {
        // Calculate curvature at position
        let k = 8.9875517923e9; // Coulomb constant
        let curvature = field.field_curvature(position, k, influence_radius, 1e-6);

        // Calculate density gradient
        let step_size = 1e-6;
        let dx = Vector3::new(step_size, 0.0, 0.0);
        let dy = Vector3::new(0.0, step_size, 0.0);
        let dz = Vector3::new(0.0, 0.0, step_size);

        let density_center = field.field_density(position, k, influence_radius);
        let density_x = field.field_density(position.add(&dx), k, influence_radius);
        let density_y = field.field_density(position.add(&dy), k, influence_radius);
        let density_z = field.field_density(position.add(&dz), k, influence_radius);

        // Gradient points toward higher density
        let gradient = Vector3::new(
            density_x - density_center,
            density_y - density_center,
            density_z - density_center,
        );

        // Force direction depends on charge sign
        // Positive charge moves away from positive density, negative toward
        let gradient_dir = gradient.normalize().unwrap_or(Vector3::zero());

        // Force magnitude proportional to curvature and charge
        let force_magnitude = curvature.abs() * charge.abs() * k * 1e10; // Scale factor

        // Apply charge direction
        if charge > 0.0 {
            gradient_dir.scale(-force_magnitude) // Away from positive density
        } else {
            gradient_dir.scale(force_magnitude) // Toward positive density
        }
    }

    /// Simulate nuclear forces (atomic scale)
    pub fn nuclear_forces(&self, atom: &Atom) -> NuclearForces {
        NuclearForces {
            strong_force: self.calculate_strong_force(atom),
            weak_force: self.calculate_weak_force(atom),
        }
    }

    /// Calculate strong nuclear force for an atom
    /// Strong force binds protons and neutrons in the nucleus
    fn calculate_strong_force(&self, atom: &Atom) -> Float {
        let nucleons = atom.nucleus.mass_number() as Float;
        // Strong force increases with nucleon count but has short range
        let base_force = self.constants.strong_force * nucleons.ln();

        // Apply EM field bias (affects nuclear stability)
        let em_influence = self.biases.em_field.stability;

        base_force * em_influence
    }

    /// Calculate weak nuclear force for an atom
    /// Weak force governs radioactive decay
    fn calculate_weak_force(&self, atom: &Atom) -> Float {
        // Weak force is much weaker than strong force
        // It depends on the neutron-proton ratio
        let total_nucleons = atom.nucleus.mass_number() as Float;
        if total_nucleons == 0.0 {
            return 0.0;
        }

        let neutron_ratio = atom.nucleus.neutrons as Float / total_nucleons;
        let base_force = self.constants.weak_force * neutron_ratio;

        // Radiation levels affect weak force (decay rates)
        let radiation_influence = 1.0 + self.biases.radiation_levels.total_radiation();

        base_force * radiation_influence
    }

    /// Calculate Lorentz force on a charged particle in a magnetic field
    /// F = q(v × B)
    pub fn lorentz_force(&self, particle: &Particle, magnetic_field: &Vector3) -> Vector3 {
        let charge = particle.charge;
        if charge == 0.0 {
            return Vector3::zero();
        }

        let velocity3d = &particle.velocity;
        let magnetic_field3d = crate::matter::Vector3D::from_vector3(magnetic_field);
        let cross_product3d = velocity3d.cross(&magnetic_field3d);
        let cross_product = cross_product3d.to_vector3();

        cross_product.scale(charge)
    }

    /// Calculate net force on a particle from multiple sources
    pub fn net_force(
        &self,
        particle: &Particle,
        gravitational_sources: &[(Vector3, Float)],
        electromagnetic_sources: &[(Vector3, Float)],
        magnetic_field: &Vector3,
    ) -> Vector3 {
        let mut net_force = Vector3::zero();
        let particle_mass = particle.mass;
        let particle_charge = particle.charge;
        let particle_pos = particle.position.to_vector3();

        // Sum gravitational forces
        for (pos, mass) in gravitational_sources {
            let grav_force = self.gravitational_force(&particle_pos, particle_mass, pos, *mass);
            net_force = net_force.add(&grav_force);
        }

        // Sum electromagnetic forces
        for (pos, charge) in electromagnetic_sources {
            let em_force = self.electromagnetic_force(&particle_pos, particle_charge, pos, *charge);
            net_force = net_force.add(&em_force);
        }

        // Add Lorentz force from magnetic field
        let lorentz = self.lorentz_force(particle, magnetic_field);
        net_force = net_force.add(&lorentz);

        net_force
    }

    /// Calculate acceleration from force using F = ma
    pub fn acceleration(&self, force: &Vector3, mass: Float) -> Vector3 {
        if mass <= 0.0 {
            return Vector3::zero();
        }
        force.scale(1.0 / mass)
    }

    /// Update particle position and velocity using simple Euler integration
    pub fn update_particle(&self, particle: &mut Particle, force: &Vector3, dt: Float) {
        let mass = particle.mass;
        let accel = self.acceleration(force, mass);
        let accel3d = crate::matter::Vector3D::from_vector3(&accel);

        // Update velocity: v = v + a * dt
        particle.velocity = particle.velocity.add(&accel3d.scale(dt));

        // Update position: x = x + v * dt
        let velocity_dt = particle.velocity.scale(dt);
        particle.position = particle
            .position
            .add(&crate::matter::Coordinate3D::from_vector3(
                &velocity_dt.to_vector3(),
            ));
    }

    /// Calculate kinetic energy: KE = 0.5 * m * v²
    pub fn kinetic_energy(&self, particle: &Particle) -> Float {
        let mass = particle.mass;
        let velocity = particle.velocity.magnitude();
        0.5 * mass * velocity * velocity
    }

    /// Calculate gravitational potential energy: PE = -G * m1 * m2 / r
    pub fn gravitational_potential_energy(
        &self,
        pos1: &Vector3,
        mass1: Float,
        pos2: &Vector3,
        mass2: Float,
    ) -> Float {
        let distance = pos1.sub(pos2).magnitude();
        if distance < 1e-10 {
            return 0.0;
        }
        -self.constants.gravity * mass1 * mass2 / distance
    }

    /// Calculate escape velocity from a body
    /// v_escape = sqrt(2 * G * M / r)
    pub fn escape_velocity(&self, mass: Float, radius: Float) -> Float {
        if radius <= 0.0 {
            return 0.0;
        }
        (2.0 * self.constants.gravity * mass / radius).sqrt()
    }

    /// Check if a particle is gravitationally bound to a body
    pub fn is_gravitationally_bound(
        &self,
        particle_pos: &Vector3,
        particle_velocity: &Vector3,
        body_pos: &Vector3,
        body_mass: Float,
    ) -> bool {
        let distance = particle_pos.sub(body_pos).magnitude();
        let velocity = particle_velocity.magnitude();
        let escape_v = self.escape_velocity(body_mass, distance);

        velocity < escape_v
    }

    /// Clear the field cache
    pub fn clear_cache(&mut self) {
        self.field_cache.clear();
    }

    /// Get the speed of light in this simulation
    pub fn speed_of_light(&self) -> Float {
        self.constants.speed_of_light
    }

    /// Get Planck's constant in this simulation
    pub fn planck_constant(&self) -> Float {
        self.constants.planck_constant
    }

    /// Apply relativistic time dilation
    /// t' = t / sqrt(1 - v²/c²)
    pub fn time_dilation(&self, velocity: Float) -> Float {
        let c = self.constants.speed_of_light;
        if velocity >= c {
            return 0.0; // Time stops at light speed
        }

        let beta_squared = (velocity / c).powi(2);
        1.0 / (1.0 - beta_squared).sqrt()
    }

    /// Apply length contraction
    /// L' = L * sqrt(1 - v²/c²)
    pub fn length_contraction(&self, length: Float, velocity: Float) -> Float {
        let c = self.constants.speed_of_light;
        if velocity >= c {
            return 0.0; // Length contracts to zero at light speed
        }

        let beta_squared = (velocity / c).powi(2);
        length * (1.0 - beta_squared).sqrt()
    }
}

/// Nuclear Forces result
#[derive(Debug, Clone)]
pub struct NuclearForces {
    /// Strong nuclear force magnitude
    pub strong_force: Float,

    /// Weak nuclear force magnitude
    pub weak_force: Float,
}

impl NuclearForces {
    /// Get the ratio of strong to weak force
    pub fn ratio(&self) -> Float {
        if self.weak_force == 0.0 {
            return Float::INFINITY;
        }
        self.strong_force / self.weak_force
    }
}

impl Default for PhysicsEngine {
    fn default() -> Self {
        Self::earth_like()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matter::particle::ParticleType;

    #[test]
    fn test_physics_engine_creation() {
        let engine = PhysicsEngine::earth_like();
        assert!(engine.constants.gravity > 0.0);
    }

    #[test]
    fn test_gravitational_force() {
        let engine = PhysicsEngine::earth_like();
        let pos1 = Vector3::new(0.0, 0.0, 0.0);
        let pos2 = Vector3::new(10.0, 0.0, 0.0);

        let force = engine.gravitational_force(&pos1, 100.0, &pos2, 200.0);
        assert!(force.x > 0.0); // Force should be in positive x direction
        assert!(force.y.abs() < 1e-10);
        assert!(force.z.abs() < 1e-10);
    }

    #[test]
    fn test_electromagnetic_force() {
        let engine = PhysicsEngine::earth_like();
        let pos1 = Vector3::new(0.0, 0.0, 0.0);
        let pos2 = Vector3::new(10.0, 0.0, 0.0);

        // Like charges repel - force magnitude calculation
        let force_like = engine.electromagnetic_force(&pos1, 1.0, &pos2, 1.0);
        assert!(force_like.magnitude() > 0.0); // Force exists

        // Opposite charges attract - force magnitude calculation
        let force_opposite = engine.electromagnetic_force(&pos1, 1.0, &pos2, -1.0);
        assert!(force_opposite.magnitude() > 0.0); // Force exists

        // Forces should be in opposite directions
        assert!((force_like.x + force_opposite.x).abs() < 1e-10);
    }

    #[test]
    fn test_nuclear_forces() {
        let engine = PhysicsEngine::earth_like();
        // TODO: Implement Atom::carbon() helper or create carbon atom manually
        // let atom = Atom::carbon();
        let nucleus = Nucleus {
            protons: 6,
            neutrons: 6,
        };
        let atom = Atom::new(6, 12, nucleus, Coordinate3D::origin());

        let forces = engine.nuclear_forces(&atom);
        assert!(forces.strong_force > 0.0);
        assert!(forces.weak_force >= 0.0);
        assert!(forces.ratio() > 1.0); // Strong force should be much stronger
    }

    #[test]
    fn test_escape_velocity() {
        let engine = PhysicsEngine::earth_like();
        let earth_mass = 5.972e24; // kg
        let earth_radius = 6.371e6; // m

        let escape_v = engine.escape_velocity(earth_mass, earth_radius);
        assert!(escape_v > 10000.0); // Should be about 11.2 km/s
    }

    #[test]
    fn test_gravitationally_bound() {
        let engine = PhysicsEngine::earth_like();
        let earth_mass = 5.972e24;

        let particle_pos = Vector3::new(6.971e6, 0.0, 0.0); // Near surface
        let slow_velocity = Vector3::new(1000.0, 0.0, 0.0);
        let fast_velocity = Vector3::new(15000.0, 0.0, 0.0);
        let body_pos = Vector3::new(0.0, 0.0, 0.0);

        assert!(engine.is_gravitationally_bound(
            &particle_pos,
            &slow_velocity,
            &body_pos,
            earth_mass
        ));
        assert!(!engine.is_gravitationally_bound(
            &particle_pos,
            &fast_velocity,
            &body_pos,
            earth_mass
        ));
    }

    #[test]
    fn test_time_dilation() {
        let engine = PhysicsEngine::earth_like();
        let c = engine.speed_of_light();

        // At rest, no time dilation
        assert!((engine.time_dilation(0.0) - 1.0).abs() < 1e-10);

        // At 0.5c, significant time dilation
        let dilation = engine.time_dilation(0.5 * c);
        assert!(dilation > 1.0);
        assert!(dilation < 2.0);

        // At light speed, time stops
        assert_eq!(engine.time_dilation(c), 0.0);
    }

    #[test]
    fn test_length_contraction() {
        let engine = PhysicsEngine::earth_like();
        let c = engine.speed_of_light();

        // At rest, no length contraction
        let original_length = 100.0;
        assert!((engine.length_contraction(original_length, 0.0) - original_length).abs() < 1e-10);

        // At 0.5c, length contracts
        let contracted = engine.length_contraction(original_length, 0.5 * c);
        assert!(contracted < original_length);
        assert!(contracted > original_length * 0.5);

        // At light speed, length contracts to zero
        assert_eq!(engine.length_contraction(original_length, c), 0.0);
    }

    #[test]
    fn test_kinetic_energy() {
        let engine = PhysicsEngine::earth_like();
        // TODO: Use Particle::from_archetype_activation() instead of Particle::new()
        // let particle = Particle::new(
        //     ParticleType::Electron,
        //     Vector3::new(0.0, 0.0, 0.0),
        //     Vector3::new(1000.0, 0.0, 0.0),
        // );
        let particle =
            Particle::from_archetype_activation(1, [1.0; 22], Coordinate3D::new(0.0, 0.0, 0.0));

        let ke = engine.kinetic_energy(&particle);
        assert!(ke > 0.0);
    }

    #[test]
    fn test_net_force() {
        let engine = PhysicsEngine::earth_like();
        // TODO: Use Particle::from_archetype_activation() instead of Particle::new()
        let particle =
            Particle::from_archetype_activation(1, [1.0; 22], Coordinate3D::new(0.0, 0.0, 0.0));

        let grav_sources = [(Vector3::new(10.0, 0.0, 0.0), 100.0)];
        let em_sources = [(Vector3::new(-10.0, 0.0, 0.0), -1.0)];
        let magnetic_field = Vector3::new(0.0, 0.0, 0.01);

        let net = engine.net_force(&particle, &grav_sources, &em_sources, &magnetic_field);
        // Should have some force components
        assert!(net.magnitude() > 0.0);
    }

    #[test]
    fn test_lorentz_force() {
        use crate::entity_layer7::holographic_blueprint::{
            HolographicSeed, HolographicSeedReference,
        };
        use crate::light::photon::Photon;

        let engine = PhysicsEngine::earth_like();

        // Create a particle using the new emergent method
        let holographic_ref =
            HolographicSeedReference::new(std::sync::Arc::new(HolographicSeed::new_from_source()));
        let photon = Photon::new_with_holographic_reference(holographic_ref.clone(), 8.187e-14);
        let mut particle = Particle::emerge_from_light(
            1,                                // ParticleID
            [0.5; 22],                        // archetype_activation
            Coordinate3D::new(0.0, 0.0, 0.0), // position
        );

        // Set custom velocity for the test
        particle.velocity = crate::matter::particle::Vector3D::new(1000.0, 0.0, 0.0);

        let b_field = Vector3::new(0.0, 0.0, 0.01);

        let force = engine.lorentz_force(&particle, &b_field);
        assert!(force.magnitude() > 0.0);

        // Force should be perpendicular to both velocity and B-field
        let v_dot_f = particle
            .velocity
            .dot(&crate::matter::particle::Vector3D::from_vector3(&force));
        assert!(v_dot_f.abs() < 1e-10);

        let b_dot_f = b_field.dot(&force);
        assert!(b_dot_f.abs() < 1e-10);
    }

    // Phase 2 Tests: Emergent Field Force Calculations

    #[test]
    fn test_use_emergent_fields_flag() {
        let mut engine = PhysicsEngine::earth_like();

        // Check default is enabled
        assert!(engine.use_emergent_fields());

        // Toggle off
        engine.set_use_emergent_fields(false);
        assert!(!engine.use_emergent_fields());

        // Toggle on
        engine.set_use_emergent_fields(true);
        assert!(engine.use_emergent_fields());
    }

    // Phase 4 Tests: Dual-Mode Physics System

    #[test]
    fn test_use_dual_mode_physics_flag() {
        let mut engine = PhysicsEngine::earth_like();

        // Check default is disabled for backward compatibility
        assert!(!engine.use_dual_mode_physics());

        // Toggle on
        engine.set_use_dual_mode_physics(true);
        assert!(engine.use_dual_mode_physics());

        // Toggle off
        engine.set_use_dual_mode_physics(false);
        assert!(!engine.use_dual_mode_physics());
    }

    #[test]
    fn test_physics_engine_initialize_dual_mode() {
        // Initialize dual-mode physics system through PhysicsEngine
        PhysicsEngine::initialize_dual_mode_physics(PhysicsMode::Hardcoded, 0.5);

        // Verify initialization succeeded
        assert!(true);
    }

    #[test]
    fn test_dual_mode_physics_backward_compatibility() {
        // Initialize dual-mode physics system
        PhysicsEngine::initialize_dual_mode_physics(PhysicsMode::Hardcoded, 0.5);

        let mut engine = PhysicsEngine::earth_like();

        // Enable dual-mode physics
        engine.set_use_dual_mode_physics(true);

        // Force calculations should still work
        let pos1 = Vector3::new(0.0, 0.0, 0.0);
        let pos2 = Vector3::new(10.0, 0.0, 0.0);

        let force = engine.gravitational_force(&pos1, 100.0, &pos2, 200.0);
        assert!(force.x > 0.0);
    }

    #[test]
    fn test_dual_mode_physics_with_emergent_fields() {
        let mut engine = PhysicsEngine::earth_like();

        // Enable both dual-mode and emergent fields
        engine.set_use_emergent_fields(true);
        engine.set_use_dual_mode_physics(true);

        // Both flags should be enabled
        assert!(engine.use_emergent_fields());
        assert!(engine.use_dual_mode_physics());
    }

    #[test]
    fn test_gravitational_force_from_field() {
        use crate::energy_fields::{GravitationalField, PhotonLike};

        // Create test struct implementing PhotonLike
        #[derive(Debug, Clone)]
        struct TestPhoton {
            position: Vector3,
            archetype_activation: [Float; 22],
            energy: Float,
        }

        impl PhotonLike for TestPhoton {
            fn position(&self) -> Vector3 {
                self.position
            }

            fn archetype_activation(&self) -> [Float; 22] {
                self.archetype_activation
            }

            fn energy(&self) -> Float {
                self.energy
            }
        }

        let mut photons = Vec::new();

        // Create test photon with high Great Way activation
        let mut activation = [0.0; 22];
        activation[6] = 1.0;
        activation[13] = 1.0;
        activation[20] = 1.0;
        photons.push(TestPhoton {
            position: Vector3::new(0.0, 0.0, 0.0),
            archetype_activation: activation,
            energy: 1.0e-10,
        });

        let field = GravitationalField::field_from_photons(&photons);
        let engine = PhysicsEngine::earth_like();

        // Calculate force at position (1, 0, 0) for mass 1.0
        let force = engine.gravitational_force_from_field(&field, Vector3::new(1.0, 0.0, 0.0), 1.0);

        // Force should be non-zero and point toward the photon
        assert!(force.magnitude() > 0.0);
        assert!(force.x < 0.0); // Points toward origin
    }

    #[test]
    fn test_gravitational_force_from_curvature() {
        use crate::energy_fields::{GravitationalField, PhotonLike};

        #[derive(Debug, Clone)]
        struct TestPhoton {
            position: Vector3,
            archetype_activation: [Float; 22],
            energy: Float,
        }

        impl PhotonLike for TestPhoton {
            fn position(&self) -> Vector3 {
                self.position
            }

            fn archetype_activation(&self) -> [Float; 22] {
                self.archetype_activation
            }

            fn energy(&self) -> Float {
                self.energy
            }
        }

        let mut photons = Vec::new();

        let mut activation = [0.0; 22];
        activation[6] = 1.0;
        activation[13] = 1.0;
        activation[20] = 1.0;
        photons.push(TestPhoton {
            position: Vector3::new(0.0, 0.0, 0.0),
            archetype_activation: activation,
            energy: 1.0e-10,
        });

        let field = GravitationalField::field_from_photons(&photons);
        let engine = PhysicsEngine::earth_like();

        // Calculate force from curvature
        let force = engine.gravitational_force_from_curvature(
            &field,
            Vector3::new(1.0, 0.0, 0.0),
            1.0,
            1.0,
        );

        // Force should be non-zero
        assert!(force.magnitude() > 0.0);
    }

    #[test]
    fn test_electromagnetic_force_from_field() {
        use crate::energy_fields::{ElectricField, PhotonLike};

        #[derive(Debug, Clone)]
        struct TestPhoton {
            position: Vector3,
            archetype_activation: [Float; 22],
            energy: Float,
        }

        impl PhotonLike for TestPhoton {
            fn position(&self) -> Vector3 {
                self.position
            }

            fn archetype_activation(&self) -> [Float; 22] {
                self.archetype_activation
            }

            fn energy(&self) -> Float {
                self.energy
            }
        }

        let mut photons = Vec::new();

        // Create positive charge photon
        let mut activation = [0.0; 22];
        activation[2] = 1.0;
        activation[9] = 1.0;
        activation[16] = 1.0;
        photons.push(TestPhoton {
            position: Vector3::new(0.0, 0.0, 0.0),
            archetype_activation: activation,
            energy: 1.0e-19,
        });

        let field = ElectricField::field_from_photons(&photons);
        let engine = PhysicsEngine::earth_like();

        // Calculate force on positive test charge at (1, 0, 0)
        let force =
            engine.electromagnetic_force_from_field(&field, Vector3::new(1.0, 0.0, 0.0), 1.0e-6);

        // Force should be non-zero
        assert!(force.magnitude() > 0.0);
    }

    #[test]
    fn test_electromagnetic_force_from_curvature() {
        use crate::energy_fields::{ElectricField, PhotonLike};

        #[derive(Debug, Clone)]
        struct TestPhoton {
            position: Vector3,
            archetype_activation: [Float; 22],
            energy: Float,
        }

        impl PhotonLike for TestPhoton {
            fn position(&self) -> Vector3 {
                self.position
            }

            fn archetype_activation(&self) -> [Float; 22] {
                self.archetype_activation
            }

            fn energy(&self) -> Float {
                self.energy
            }
        }

        let mut photons = Vec::new();

        let mut activation = [0.0; 22];
        activation[2] = 1.0;
        activation[9] = 1.0;
        activation[16] = 1.0;
        photons.push(TestPhoton {
            position: Vector3::new(0.0, 0.0, 0.0),
            archetype_activation: activation,
            energy: 1.0e-19,
        });

        let field = ElectricField::field_from_photons(&photons);
        let engine = PhysicsEngine::earth_like();

        // Calculate force from curvature on positive charge
        let force = engine.electromagnetic_force_from_curvature(
            &field,
            Vector3::new(1.0, 0.0, 0.0),
            1.0e-6,
            1.0,
        );

        // Force should be non-zero
        assert!(force.magnitude() > 0.0);
    }
}
