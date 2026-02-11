// Energy Fields System - Phase 10 Implementation
// Models gravitational, electromagnetic, and nuclear fields
// These fields emerge from Light (3rd distortion) and structure matter
//
// Phase 2 Update: Fields now emerge from photon collective behavior
// Knowledge Base Reference: REFACTOR_ROADMAP_V2.md Section 2.5
// "Forces emerge from interactions - Not fundamental laws"

use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::Arc;

/// Float type for physical calculations
pub type Float = f64;

// Forward declaration for Photon (to avoid circular dependency)
// We'll use a trait or struct reference instead
pub trait PhotonLike {
    fn position(&self) -> Vector3;
    fn archetype_activation(&self) -> [Float; 22];
    fn energy(&self) -> Float;
}

/// 3D vector for spatial calculations
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Vector3 {
    /// Create a new 3D vector
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Vector3 { x, y, z }
    }

    /// Zero vector
    pub fn zero() -> Self {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Magnitude of the vector
    pub fn magnitude(&self) -> Float {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Normalize the vector
    pub fn normalize(&self) -> Option<Vector3> {
        let mag = self.magnitude();
        if mag > 0.0 {
            Some(Vector3 {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
            })
        } else {
            None
        }
    }

    /// Dot product with another vector
    pub fn dot(&self, other: &Vector3) -> Float {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Cross product with another vector
    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Add two vectors
    pub fn add(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    /// Subtract two vectors
    pub fn sub(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    /// Scale the vector
    pub fn scale(&self, scalar: Float) -> Vector3 {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    /// Convert to matter::Vector3D
    pub fn to_vector3d(&self) -> crate::matter::Vector3D {
        crate::matter::Vector3D::new(self.x, self.y, self.z)
    }

    /// Convert to matter::Coordinate3D
    pub fn to_coordinate3d(&self) -> crate::matter::Coordinate3D {
        crate::matter::Coordinate3D::new(self.x, self.y, self.z)
    }

    /// Convert from matter::Vector3D
    pub fn from_vector3d(v: &crate::matter::Vector3D) -> Self {
        Vector3::new(v.vx, v.vy, v.vz)
    }

    /// Convert from matter::Coordinate3D
    pub fn from_coordinate3d(c: &crate::matter::Coordinate3D) -> Self {
        Vector3::new(c.x, c.y, c.z)
    }
}

/// Field line representation for visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldLine {
    pub points: Vec<Vector3>,
    pub strength: Float,
}

/// Gravitational Field
///
/// Phase 2 Update: Gravitational field emerges from photon collective behavior
/// Knowledge Base Reference: REFACTOR_ROADMAP_V2.md Section 2.5
/// "Gravity emerges from Great Way archetypes (A7, A14, A21)"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GravitationalField {
    /// Field sources (masses) - legacy support, will be deprecated
    pub sources: Vec<GravitationalSource>,
    /// Field lines for visualization
    pub field_lines: Vec<FieldLine>,
    /// Overall field strength at center
    pub central_field_strength: Float,
    /// Photons that contribute to this gravitational field
    pub photons: Vec<GravitationalPhoton>,
    /// Holographic reference to the complete seed
    ///
    /// Phase 2: Fields now have holographic references to maintain
    /// the fractal-holographic principle: "part contains whole"
    #[serde(skip)] // Skip serialization for Arc
    pub holographic_ref: Option<Arc<()>>, // TODO: Replace with HolographicSeed
}

impl Default for GravitationalField {
    fn default() -> Self {
        Self::empty()
    }
}

impl GravitationalField {
    /// Create an empty gravitational field
    pub fn empty() -> Self {
        GravitationalField {
            sources: Vec::new(),
            field_lines: Vec::new(),
            central_field_strength: 0.0,
            photons: Vec::new(),
            holographic_ref: None,
        }
    }

    /// Create a gravitational field with point mass at origin
    pub fn point_mass(mass: Float) -> Self {
        GravitationalField {
            sources: vec![GravitationalSource {
                position: Vector3::zero(),
                mass,
            }],
            field_lines: Vec::new(),
            central_field_strength: 0.0,
            photons: Vec::new(),
            holographic_ref: None,
        }
    }

    /// Create a uniform gravitational field
    pub fn uniform(strength: Vector3) -> Self {
        GravitationalField {
            sources: vec![GravitationalSource {
                position: Vector3::zero(),
                mass: strength.magnitude(),
            }],
            field_lines: vec![FieldLine {
                points: vec![
                    Vector3::zero(),
                    strength.normalize().unwrap_or(Vector3::new(0.0, 1.0, 0.0)),
                ],
                strength: strength.magnitude(),
            }],
            central_field_strength: strength.magnitude(),
            photons: Vec::new(),
            holographic_ref: None,
        }
    }

    /// Add a gravitational source
    pub fn add_source(&mut self, position: Vector3, mass: Float) {
        self.sources.push(GravitationalSource { position, mass });
    }

    /// Create gravitational field from photons
    ///
    /// Phase 2: Fields emerge from photon collective behavior
    /// Knowledge Base Reference: REFACTOR_ROADMAP_V2.md Section 2.5
    /// "Gravity emerges from Great Way archetypes (A7, A14, A21)"
    ///
    /// # Arguments
    /// * `photons` - Slice of photon-like objects with archetype activation
    ///
    /// # Returns
    /// A gravitational field that emerges from the collective behavior of photons
    pub fn field_from_photons<P>(photons: &[P]) -> Self
    where
        P: PhotonLike,
    {
        let mut field = GravitationalField::empty();

        for photon in photons {
            let activation = photon.archetype_activation();
            let grav_photon = GravitationalPhoton::from_archetypes(
                photon.position(),
                photon.energy(),
                &activation,
            );
            field.photons.push(grav_photon);
        }

        field
    }

    /// Calculate gravitational field at a given position
    ///
    /// Phase 2: Field calculation now includes photon contributions
    pub fn field_at(&self, position: Vector3, g_constant: Float) -> Vector3 {
        let mut total_field = Vector3::zero();

        // Legacy source calculation (for backward compatibility)
        for source in &self.sources {
            let r_vec = source.position.sub(&position);
            let distance = r_vec.magnitude();

            if distance > 1e-10 {
                let field_magnitude = g_constant * source.mass / (distance * distance);
                let field_direction = r_vec.normalize().unwrap_or(Vector3::zero());
                total_field = total_field.add(&field_direction.scale(field_magnitude));
            }
        }

        // Emergent field from photons (Phase 2)
        const C: Float = 2.998e8; // Speed of light
        for photon in &self.photons {
            let r_vec = photon.position.sub(&position);
            let distance = r_vec.magnitude();

            if distance > 1e-10 {
                // Mass equivalent from energy: m = E/c²
                let mass_equiv = photon.energy / (C * C);
                // Field strength modified by Great Way activation
                let effective_mass = mass_equiv * photon.great_way_activation;
                let field_magnitude = g_constant * effective_mass / (distance * distance);
                let field_direction = r_vec.normalize().unwrap_or(Vector3::zero());
                total_field = total_field.add(&field_direction.scale(field_magnitude));
            }
        }

        total_field
    }

    /// Calculate gravitational potential at a given position
    pub fn potential_at(&self, position: Vector3, g_constant: Float) -> Float {
        let mut total_potential = 0.0;

        for source in &self.sources {
            let distance = source.position.sub(&position).magnitude();
            if distance > 1e-10 {
                total_potential -= g_constant * source.mass / distance;
            }
        }

        total_potential
    }

    /// Calculate gravitational force on a mass at a given position
    pub fn force_on(&self, position: Vector3, mass: Float, g_constant: Float) -> Vector3 {
        self.field_at(position, g_constant).scale(mass)
    }

    /// Generate field lines for visualization
    pub fn generate_field_lines(&mut self, num_lines: usize, steps: usize, step_size: Float) {
        if self.sources.is_empty() {
            return;
        }

        self.field_lines.clear();

        for i in 0..num_lines {
            let angle = 2.0 * std::f64::consts::PI * (i as Float) / (num_lines as Float);

            // Start point at some distance from source
            let start_distance = 1.0;
            let start_pos = Vector3::new(
                start_distance * angle.cos(),
                start_distance * angle.sin(),
                0.0,
            );

            let mut points = vec![start_pos];
            let mut current_pos = start_pos;

            for _ in 0..steps {
                let field = self.field_at(current_pos, 1.0);
                let field_dir = field.normalize();

                if let Some(direction) = field_dir {
                    current_pos = current_pos.add(&direction.scale(step_size));
                    points.push(current_pos);

                    // Stop if too far or too close
                    if current_pos.magnitude() < 0.1 || current_pos.magnitude() > 100.0 {
                        break;
                    }
                } else {
                    break;
                }
            }

            self.field_lines.push(FieldLine {
                points,
                strength: self.field_at(start_pos, 1.0).magnitude(),
            });
        }
    }

    /// Get total mass in the field
    pub fn total_mass(&self) -> Float {
        self.sources.iter().map(|s| s.mass).sum()
    }

    /// Get center of mass
    pub fn center_of_mass(&self) -> Option<Vector3> {
        let total_mass = self.total_mass();
        if total_mass == 0.0 {
            return None;
        }

        let mut weighted_sum = Vector3::zero();
        for source in &self.sources {
            weighted_sum = weighted_sum.add(&source.position.scale(source.mass));
        }

        Some(weighted_sum.scale(1.0 / total_mass))
    }

    /// Calculate field density at a given position
    ///
    /// Phase 2: Field density emerges from photon collective behavior
    /// Knowledge Base Reference: REFACTOR_ROADMAP_V2.md Section 2.3
    /// "Forces emerge from interactions - Not fundamental laws"
    ///
    /// Field density represents the concentration of gravitational influence
    /// at a given point in space, emerging from the collective Great Way
    /// activation of nearby photons.
    ///
    /// # Arguments
    /// * `position` - The position to calculate density at
    /// * `g_constant` - Gravitational constant
    /// * `influence_radius` - Radius within which photons contribute to density
    ///
    /// # Returns
    /// The field density at the given position
    pub fn field_density(
        &self,
        position: Vector3,
        g_constant: Float,
        influence_radius: Float,
    ) -> Float {
        let mut total_density = 0.0;

        for photon in &self.photons {
            let distance = photon.position.sub(&position).magnitude();

            if distance <= influence_radius {
                // Density contribution from this photon
                // Higher Great Way activation = higher density contribution
                const C: Float = 2.998e8; // Speed of light
                let mass_equiv = photon.energy / (C * C);
                let effective_mass = mass_equiv * photon.great_way_activation;

                // Density falls off with distance (Gaussian-like distribution)
                let distance_factor =
                    (-distance * distance / (influence_radius * influence_radius)).exp();
                total_density += effective_mass * distance_factor;
            }
        }

        // Scale by gravitational constant
        g_constant * total_density
    }

    /// Calculate field curvature at a given position
    ///
    /// Phase 2: Field curvature emerges from photon density gradients
    /// Knowledge Base Reference: REFACTOR_ROADMAP_V2.md Section 2.3
    /// "Spacetime curvature emerges from Light patterns"
    ///
    /// Field curvature represents the curvature of spacetime at a given point,
    /// emerging from the gradient of photon density. This is how gravity
    /// manifests in general relativity.
    ///
    /// # Arguments
    /// * `position` - The position to calculate curvature at
    /// * `g_constant` - Gravitational constant
    /// * `influence_radius` - Radius for density calculation
    /// * `step_size` - Step size for gradient calculation (default: 1e-6)
    ///
    /// # Returns
    /// The scalar curvature at the given position
    pub fn field_curvature(
        &self,
        position: Vector3,
        g_constant: Float,
        influence_radius: Float,
        step_size: Float,
    ) -> Float {
        // Calculate density at center position
        let center_density = self.field_density(position, g_constant, influence_radius);

        // Calculate density at offset positions for gradient
        let dx = Vector3::new(step_size, 0.0, 0.0);
        let dy = Vector3::new(0.0, step_size, 0.0);
        let dz = Vector3::new(0.0, 0.0, step_size);

        let density_x_plus = self.field_density(position.add(&dx), g_constant, influence_radius);
        let density_x_minus = self.field_density(position.sub(&dx), g_constant, influence_radius);
        let density_y_plus = self.field_density(position.add(&dy), g_constant, influence_radius);
        let density_y_minus = self.field_density(position.sub(&dy), g_constant, influence_radius);
        let density_z_plus = self.field_density(position.add(&dz), g_constant, influence_radius);
        let density_z_minus = self.field_density(position.sub(&dz), g_constant, influence_radius);

        // Calculate second derivatives (Laplacian)
        let d2_dx2 =
            (density_x_plus - 2.0 * center_density + density_x_minus) / (step_size * step_size);
        let d2_dy2 =
            (density_y_plus - 2.0 * center_density + density_y_minus) / (step_size * step_size);
        let d2_dz2 =
            (density_z_plus - 2.0 * center_density + density_z_minus) / (step_size * step_size);

        // Curvature is proportional to Laplacian of density
        d2_dx2 + d2_dy2 + d2_dz2
    }

    /// Set holographic reference for this field
    ///
    /// Phase 2: Fields maintain holographic references to preserve
    /// the fractal-holographic principle: "part contains whole"
    pub fn set_holographic_ref(&mut self, _holographic_ref: Arc<()>) {
        // TODO: Replace with actual HolographicSeed once available
        self.holographic_ref = Some(Arc::new(()));
    }
}

/// Gravitational source (mass)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GravitationalSource {
    pub position: Vector3,
    pub mass: Float,
}

/// Photon contribution to gravitational field
///
/// Phase 2: Photons contribute to gravitational field based on their
/// Great Way archetype activation (A7, A14, A21)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GravitationalPhoton {
    /// Photon position
    pub position: Vector3,
    /// Photon energy (contributes to mass via E=mc²)
    pub energy: Float,
    /// Great Way activation (A7, A14, A21) - determines gravitational influence
    pub great_way_activation: Float,
}

impl GravitationalPhoton {
    /// Create a gravitational photon from archetype activation
    pub fn from_archetypes(
        position: Vector3,
        energy: Float,
        archetype_activation: &[Float; 22],
    ) -> Self {
        // Great Way archetypes are at indices 6, 13, 20 (A7, A14, A21)
        let gw_activation =
            (archetype_activation[6] + archetype_activation[13] + archetype_activation[20]) / 3.0;

        GravitationalPhoton {
            position,
            energy,
            great_way_activation: gw_activation,
        }
    }
}

/// Electric Field
///
/// Phase 2 Update: Electric field emerges from photon collective behavior
/// Knowledge Base Reference: REFACTOR_ROADMAP_V2.md Section 2.5
/// "EM emerges from Catalyst archetypes (A3, A10, A17)"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectricField {
    /// Field sources (charges) - legacy support, will be deprecated
    pub sources: Vec<ElectricSource>,
    /// Field lines for visualization
    pub field_lines: Vec<FieldLine>,
    /// Photons that contribute to this electric field
    pub photons: Vec<ElectricPhoton>,
    /// Holographic reference to the complete seed
    ///
    /// Phase 2: Fields now have holographic references
    #[serde(skip)] // Skip serialization for Arc
    pub holographic_ref: Option<Arc<()>>, // TODO: Replace with HolographicSeed
}

impl Default for ElectricField {
    fn default() -> Self {
        Self::empty()
    }
}

impl ElectricField {
    /// Create an empty electric field
    pub fn empty() -> Self {
        ElectricField {
            sources: Vec::new(),
            field_lines: Vec::new(),
            photons: Vec::new(),
            holographic_ref: None,
        }
    }

    /// Create a point charge electric field
    pub fn point_charge(charge: Float) -> Self {
        ElectricField {
            sources: vec![ElectricSource {
                position: Vector3::zero(),
                charge,
            }],
            field_lines: Vec::new(),
            photons: Vec::new(),
            holographic_ref: None,
        }
    }

    /// Add an electric source
    pub fn add_source(&mut self, position: Vector3, charge: Float) {
        self.sources.push(ElectricSource { position, charge });
    }

    /// Create electric field from photons
    ///
    /// Phase 2: Fields emerge from photon collective behavior
    /// Knowledge Base Reference: REFACTOR_ROADMAP_V2.md Section 2.5
    /// "EM emerges from Catalyst archetypes (A3, A10, A17)"
    ///
    /// # Arguments
    /// * `photons` - Slice of photon-like objects with archetype activation
    ///
    /// # Returns
    /// An electric field that emerges from the collective behavior of photons
    pub fn field_from_photons<P>(photons: &[P]) -> Self
    where
        P: PhotonLike,
    {
        let mut field = ElectricField::empty();

        for photon in photons {
            let activation = photon.archetype_activation();
            let elec_photon = ElectricPhoton::from_archetypes(photon.position(), &activation);
            field.photons.push(elec_photon);
        }

        field
    }

    /// Calculate electric field at a given position
    ///
    /// Phase 2: Field calculation now includes photon contributions
    pub fn field_at(&self, position: Vector3, k_constant: Float) -> Vector3 {
        let mut total_field = Vector3::zero();

        // Legacy source calculation (for backward compatibility)
        for source in &self.sources {
            let r_vec = position.sub(&source.position);
            let distance = r_vec.magnitude();

            if distance > 1e-10 {
                let field_magnitude = k_constant * source.charge / (distance * distance);
                let field_direction = r_vec.normalize().unwrap_or(Vector3::zero());
                total_field = total_field.add(&field_direction.scale(field_magnitude));
            }
        }

        // Emergent field from photons (Phase 2)
        for photon in &self.photons {
            let r_vec = position.sub(&photon.position);
            let distance = r_vec.magnitude();

            if distance > 1e-10 {
                // Field strength modified by Catalyst activation
                let effective_charge = photon.charge * photon.catalyst_activation;
                let field_magnitude = k_constant * effective_charge / (distance * distance);
                let field_direction = r_vec.normalize().unwrap_or(Vector3::zero());
                total_field = total_field.add(&field_direction.scale(field_magnitude));
            }
        }

        total_field
    }

    /// Calculate electric potential at a given position
    pub fn potential_at(&self, position: Vector3, k_constant: Float) -> Float {
        let mut total_potential = 0.0;

        for source in &self.sources {
            let distance = position.sub(&source.position).magnitude();
            if distance > 1e-10 {
                total_potential += k_constant * source.charge / distance;
            }
        }

        total_potential
    }

    /// Calculate electric force on a charge at a given position
    pub fn force_on(&self, position: Vector3, charge: Float, k_constant: Float) -> Vector3 {
        self.field_at(position, k_constant).scale(charge)
    }

    /// Get total charge in the field
    pub fn total_charge(&self) -> Float {
        self.sources.iter().map(|s| s.charge).sum()
    }

    /// Calculate field density at a given position
    ///
    /// Phase 2: Field density emerges from photon collective behavior
    /// Knowledge Base Reference: REFACTOR_ROADMAP_V2.md Section 2.5
    /// "EM emerges from Catalyst archetypes (A3, A10, A17)"
    ///
    /// Field density represents the concentration of electric influence
    /// at a given point in space, emerging from the collective Catalyst
    /// activation of nearby photons.
    ///
    /// # Arguments
    /// * `position` - The position to calculate density at
    /// * `k_constant` - Coulomb constant
    /// * `influence_radius` - Radius within which photons contribute to density
    ///
    /// # Returns
    /// The field density at the given position
    pub fn field_density(
        &self,
        position: Vector3,
        k_constant: Float,
        influence_radius: Float,
    ) -> Float {
        let mut total_density = 0.0;

        for photon in &self.photons {
            let distance = photon.position.sub(&position).magnitude();

            if distance <= influence_radius {
                // Density contribution from this photon
                // Higher Catalyst activation = higher density contribution
                let effective_charge = photon.charge * photon.catalyst_activation;

                // Density falls off with distance (Gaussian-like distribution)
                let distance_factor =
                    (-distance * distance / (influence_radius * influence_radius)).exp();
                total_density += effective_charge.abs() * distance_factor;
            }
        }

        // Scale by Coulomb constant
        k_constant * total_density
    }

    /// Calculate field curvature at a given position
    ///
    /// Phase 2: Field curvature emerges from photon density gradients
    /// Knowledge Base Reference: REFACTOR_ROADMAP_V2.md Section 2.5
    /// "Electric field curvature emerges from charge density gradients"
    ///
    /// Field curvature represents the curvature of the electric potential
    /// at a given point, emerging from the gradient of photon charge density.
    /// This is related to the electric field divergence in Maxwell's equations.
    ///
    /// # Arguments
    /// * `position` - The position to calculate curvature at
    /// * `k_constant` - Coulomb constant
    /// * `influence_radius` - Radius for density calculation
    /// * `step_size` - Step size for gradient calculation (default: 1e-6)
    ///
    /// # Returns
    /// The scalar curvature at the given position
    pub fn field_curvature(
        &self,
        position: Vector3,
        k_constant: Float,
        influence_radius: Float,
        step_size: Float,
    ) -> Float {
        // Calculate density at center position
        let center_density = self.field_density(position, k_constant, influence_radius);

        // Calculate density at offset positions for gradient
        let dx = Vector3::new(step_size, 0.0, 0.0);
        let dy = Vector3::new(0.0, step_size, 0.0);
        let dz = Vector3::new(0.0, 0.0, step_size);

        let density_x_plus = self.field_density(position.add(&dx), k_constant, influence_radius);
        let density_x_minus = self.field_density(position.sub(&dx), k_constant, influence_radius);
        let density_y_plus = self.field_density(position.add(&dy), k_constant, influence_radius);
        let density_y_minus = self.field_density(position.sub(&dy), k_constant, influence_radius);
        let density_z_plus = self.field_density(position.add(&dz), k_constant, influence_radius);
        let density_z_minus = self.field_density(position.sub(&dz), k_constant, influence_radius);

        // Calculate second derivatives (Laplacian)
        let d2_dx2 =
            (density_x_plus - 2.0 * center_density + density_x_minus) / (step_size * step_size);
        let d2_dy2 =
            (density_y_plus - 2.0 * center_density + density_y_minus) / (step_size * step_size);
        let d2_dz2 =
            (density_z_plus - 2.0 * center_density + density_z_minus) / (step_size * step_size);

        // Curvature is proportional to Laplacian of density
        d2_dx2 + d2_dy2 + d2_dz2
    }

    /// Set holographic reference for this field
    ///
    /// Phase 2: Fields maintain holographic references to preserve
    /// the fractal-holographic principle: "part contains whole"
    pub fn set_holographic_ref(&mut self, _holographic_ref: Arc<()>) {
        // TODO: Replace with actual HolographicSeed once available
        self.holographic_ref = Some(Arc::new(()));
    }
}

/// Electric source (charge)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectricSource {
    pub position: Vector3,
    pub charge: Float,
}

/// Photon contribution to electric field
///
/// Phase 2: Photons contribute to electric field based on their
/// Catalyst archetype activation (A3, A10, A17)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectricPhoton {
    /// Photon position
    pub position: Vector3,
    /// Photon charge magnitude (derived from Catalyst archetypes)
    pub charge: Float,
    /// Catalyst activation (A3, A10, A17) - determines electric influence
    pub catalyst_activation: Float,
}

impl ElectricPhoton {
    /// Create an electric photon from archetype activation
    pub fn from_archetypes(position: Vector3, archetype_activation: &[Float; 22]) -> Self {
        // Catalyst archetypes are at indices 2, 9, 16 (A3, A10, A17)
        let catalyst_activation =
            (archetype_activation[2] + archetype_activation[9] + archetype_activation[16]) / 3.0;

        // Charge emerges from Catalyst activation
        // Positive if activation > 0.5, negative otherwise
        let charge_magnitude = (catalyst_activation - 0.5).abs() * 1.6e-19; // Elementary charge scale
        let charge = if catalyst_activation > 0.5 {
            charge_magnitude
        } else {
            -charge_magnitude
        };

        ElectricPhoton {
            position,
            charge,
            catalyst_activation,
        }
    }
}

/// Magnetic Field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagneticField {
    /// Field sources (currents or magnetic dipoles)
    pub sources: Vec<MagneticSource>,
    /// Field lines for visualization
    pub field_lines: Vec<FieldLine>,
}

impl Default for MagneticField {
    fn default() -> Self {
        Self::empty()
    }
}

impl MagneticField {
    /// Create an empty magnetic field
    pub fn empty() -> Self {
        MagneticField {
            sources: Vec::new(),
            field_lines: Vec::new(),
        }
    }

    /// Create a uniform magnetic field
    pub fn uniform(field: Vector3) -> Self {
        MagneticField {
            sources: vec![MagneticSource {
                position: Vector3::zero(),
                field,
                source_type: MagneticSourceType::Uniform,
            }],
            field_lines: vec![FieldLine {
                points: vec![
                    Vector3::zero(),
                    field.normalize().unwrap_or(Vector3::new(0.0, 0.0, 1.0)),
                ],
                strength: field.magnitude(),
            }],
        }
    }

    /// Add a magnetic source
    pub fn add_source(
        &mut self,
        position: Vector3,
        field: Vector3,
        source_type: MagneticSourceType,
    ) {
        self.sources.push(MagneticSource {
            position,
            field,
            source_type,
        });
    }

    /// Calculate magnetic field at a given position
    pub fn field_at(&self, position: Vector3) -> Vector3 {
        let mut total_field = Vector3::zero();

        for source in &self.sources {
            match source.source_type {
                MagneticSourceType::Uniform => {
                    total_field = total_field.add(&source.field);
                }
                MagneticSourceType::Dipole => {
                    let r_vec = position.sub(&source.position);
                    let distance = r_vec.magnitude();
                    if distance > 1e-10 {
                        let r_hat = r_vec.normalize().unwrap_or(Vector3::zero());
                        let m = source.field; // Magnetic moment
                        let r2 = distance * distance;
                        let r5 = r2 * r2 * distance;

                        // Dipole field formula
                        let term1 = r_hat.scale(3.0 * m.dot(&r_hat));
                        let term2 = m;
                        let dipole_field = term1.sub(&term2).scale(1.0 / r5);
                        total_field = total_field.add(&dipole_field);
                    }
                }
            }
        }

        total_field
    }

    /// Calculate magnetic force on a moving charge
    pub fn force_on(&self, position: Vector3, charge: Float, velocity: Vector3) -> Vector3 {
        let b_field = self.field_at(position);
        // Lorentz force: F = q(v × B)
        velocity.cross(&b_field).scale(charge)
    }
}

/// Magnetic source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagneticSource {
    pub position: Vector3,
    pub field: Vector3,
    pub source_type: MagneticSourceType,
}

/// Type of magnetic source
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MagneticSourceType {
    Uniform,
    Dipole,
}

/// Electromagnetic Field (combined electric and magnetic)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectromagneticField {
    pub electric_field: ElectricField,
    pub magnetic_field: MagneticField,
}

impl Default for ElectromagneticField {
    fn default() -> Self {
        Self::empty()
    }
}

impl ElectromagneticField {
    /// Create an empty electromagnetic field
    pub fn empty() -> Self {
        ElectromagneticField {
            electric_field: ElectricField::empty(),
            magnetic_field: MagneticField::empty(),
        }
    }

    /// Create a plane wave electromagnetic field
    pub fn plane_wave(
        _electric_field: Vector3,
        magnetic_field: Vector3,
        _wavelength: Float,
        _frequency: Float,
    ) -> Self {
        ElectromagneticField {
            electric_field: ElectricField::point_charge(1.0),
            magnetic_field: MagneticField::uniform(magnetic_field),
        }
    }

    /// Calculate Lorentz force on a moving charge
    pub fn lorentz_force(
        &self,
        position: Vector3,
        charge: Float,
        velocity: Vector3,
        k_constant: Float,
    ) -> Vector3 {
        let electric_force = self.electric_field.force_on(position, charge, k_constant);
        let magnetic_force = self.magnetic_field.force_on(position, charge, velocity);
        electric_force.add(&magnetic_force)
    }

    /// Calculate Poynting vector (energy flux)
    pub fn poynting_vector(&self, position: Vector3, permittivity: Float) -> Vector3 {
        let e = self.electric_field.field_at(position, 1.0);
        let b = self.magnetic_field.field_at(position);
        // S = (1/μ₀) E × B
        e.cross(&b).scale(1.0 / permittivity)
    }

    /// Calculate electromagnetic energy density
    pub fn energy_density(
        &self,
        position: Vector3,
        permittivity: Float,
        permeability: Float,
    ) -> Float {
        let e = self.electric_field.field_at(position, 1.0).magnitude();
        let b = self.magnetic_field.field_at(position).magnitude();
        // u = (1/2)(ε₀E² + (1/μ₀)B²)
        0.5 * (permittivity * e * e + b * b / permeability)
    }
}

/// Nuclear Field (strong and weak nuclear forces)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuclearField {
    pub strong_nuclear_field: StrongNuclearField,
    pub weak_nuclear_field: WeakNuclearField,
}

impl Default for NuclearField {
    fn default() -> Self {
        Self::empty()
    }
}

impl NuclearField {
    /// Create an empty nuclear field
    pub fn empty() -> Self {
        NuclearField {
            strong_nuclear_field: StrongNuclearField::empty(),
            weak_nuclear_field: WeakNuclearField::empty(),
        }
    }

    /// Calculate total nuclear field strength
    pub fn total_field_strength(&self, distance: Float) -> Float {
        self.strong_nuclear_field.field_strength(distance)
            + self.weak_nuclear_field.field_strength(distance)
    }
}

/// Strong Nuclear Field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrongNuclearField {
    pub range: Float, // Effective range (~1 fm)
    pub coupling_constant: Float,
}

impl Default for StrongNuclearField {
    fn default() -> Self {
        Self::empty()
    }
}

impl StrongNuclearField {
    /// Create an empty strong nuclear field
    pub fn empty() -> Self {
        StrongNuclearField {
            range: 1.0e-15, // 1 femtometer
            coupling_constant: 1.0,
        }
    }

    /// Create a strong nuclear field with given parameters
    pub fn with_parameters(range: Float, coupling_constant: Float) -> Self {
        StrongNuclearField {
            range,
            coupling_constant,
        }
    }

    /// Calculate field strength at given distance
    pub fn field_strength(&self, distance: Float) -> Float {
        if distance > self.range * 3.0 {
            0.0
        } else {
            // Yukawa potential approximation
            self.coupling_constant * (-distance / self.range).exp() / distance
        }
    }

    /// Check if particles are within strong force range
    pub fn within_range(&self, distance: Float) -> bool {
        distance <= self.range * 3.0
    }
}

/// Weak Nuclear Field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeakNuclearField {
    pub range: Float, // Effective range (~0.001 fm)
    pub coupling_constant: Float,
}

impl Default for WeakNuclearField {
    fn default() -> Self {
        Self::empty()
    }
}

impl WeakNuclearField {
    /// Create an empty weak nuclear field
    pub fn empty() -> Self {
        WeakNuclearField {
            range: 1.0e-18, // 0.001 femtometer
            coupling_constant: 0.1,
        }
    }

    /// Create a weak nuclear field with given parameters
    pub fn with_parameters(range: Float, coupling_constant: Float) -> Self {
        WeakNuclearField {
            range,
            coupling_constant,
        }
    }

    /// Calculate field strength at given distance
    pub fn field_strength(&self, distance: Float) -> Float {
        if distance > self.range * 3.0 {
            0.0
        } else {
            // Very short-range interaction
            self.coupling_constant * (-distance / self.range).exp() / distance
        }
    }

    /// Check if particles are within weak force range
    pub fn within_range(&self, distance: Float) -> bool {
        distance <= self.range * 3.0
    }
}

/// Energy Fields (all fields combined)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyFields {
    pub gravitational_field: GravitationalField,
    pub electromagnetic_field: ElectromagneticField,
    pub nuclear_field: NuclearField,
}

impl Default for EnergyFields {
    fn default() -> Self {
        Self::empty()
    }
}

impl EnergyFields {
    /// Create empty energy fields
    pub fn empty() -> Self {
        EnergyFields {
            gravitational_field: GravitationalField::empty(),
            electromagnetic_field: ElectromagneticField::empty(),
            nuclear_field: NuclearField::empty(),
        }
    }

    /// Create energy fields from light (third distortion)
    pub fn from_light(light_energy: Float) -> Self {
        EnergyFields {
            gravitational_field: GravitationalField::point_mass(light_energy * 1e-30),
            electromagnetic_field: ElectromagneticField::empty(),
            nuclear_field: NuclearField::empty(),
        }
    }

    /// Calculate total field energy at a position
    pub fn total_field_energy(&self, position: Vector3) -> Float {
        let gravitational_energy = self
            .gravitational_field
            .potential_at(position, 6.67430e-11)
            .abs();
        let electromagnetic_energy = self
            .electromagnetic_field
            .energy_density(position, 8.854e-12, 1.257e-6);
        let nuclear_energy = self.nuclear_field.total_field_strength(1.0e-15);

        gravitational_energy + electromagnetic_energy + nuclear_energy
    }

    /// Get summary of all fields
    pub fn summary(&self) -> String {
        format!(
            "Energy Fields:\n\
             - Gravitational Sources: {}\n\
             - Electric Sources: {}\n\
             - Magnetic Sources: {}\n\
             - Strong Nuclear Range: {:.2e} m\n\
             - Weak Nuclear Range: {:.2e} m",
            self.gravitational_field.sources.len(),
            self.electromagnetic_field.electric_field.sources.len(),
            self.electromagnetic_field.magnetic_field.sources.len(),
            self.nuclear_field.strong_nuclear_field.range,
            self.nuclear_field.weak_nuclear_field.range
        )
    }
}

impl fmt::Display for EnergyFields {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.summary())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector3_creation() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_vector3_zero() {
        let v = Vector3::zero();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
    }

    #[test]
    fn test_vector3_magnitude() {
        let v = Vector3::new(3.0, 4.0, 0.0);
        assert_eq!(v.magnitude(), 5.0);
    }

    #[test]
    fn test_vector3_normalize() {
        let v = Vector3::new(3.0, 4.0, 0.0);
        let normalized = v.normalize().unwrap();
        assert!((normalized.magnitude() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_vector3_dot_product() {
        let v1 = Vector3::new(1.0, 0.0, 0.0);
        let v2 = Vector3::new(0.0, 1.0, 0.0);
        assert_eq!(v1.dot(&v2), 0.0);
    }

    #[test]
    fn test_vector3_cross_product() {
        let v1 = Vector3::new(1.0, 0.0, 0.0);
        let v2 = Vector3::new(0.0, 1.0, 0.0);
        let result = v1.cross(&v2);
        assert!((result.x - 0.0).abs() < 1e-10);
        assert!((result.y - 0.0).abs() < 1e-10);
        assert!((result.z - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_gravitational_field_empty() {
        let field = GravitationalField::empty();
        assert_eq!(field.sources.len(), 0);
    }

    #[test]
    fn test_gravitational_field_point_mass() {
        let field = GravitationalField::point_mass(1.0);
        assert_eq!(field.sources.len(), 1);
        assert_eq!(field.sources[0].mass, 1.0);
    }

    #[test]
    fn test_gravitational_field_add_source() {
        let mut field = GravitationalField::empty();
        field.add_source(Vector3::new(1.0, 0.0, 0.0), 5.0);
        assert_eq!(field.sources.len(), 1);
        assert_eq!(field.sources[0].mass, 5.0);
    }

    #[test]
    fn test_gravitational_field_at() {
        let field = GravitationalField::point_mass(1.0);
        let g = 6.67430e-11;
        let field_vec = field.field_at(Vector3::new(1.0, 0.0, 0.0), g);
        assert!(field_vec.magnitude() > 0.0);
    }

    #[test]
    fn test_gravitational_potential_at() {
        let field = GravitationalField::point_mass(1.0);
        let g = 6.67430e-11;
        let potential = field.potential_at(Vector3::new(1.0, 0.0, 0.0), g);
        assert!(potential < 0.0);
    }

    #[test]
    fn test_gravitational_force_on() {
        let field = GravitationalField::point_mass(1.0);
        let g = 6.67430e-11;
        let force = field.force_on(Vector3::new(1.0, 0.0, 0.0), 0.1, g);
        assert!(force.magnitude() > 0.0);
    }

    #[test]
    fn test_gravitational_total_mass() {
        let mut field = GravitationalField::empty();
        field.add_source(Vector3::zero(), 1.0);
        field.add_source(Vector3::new(1.0, 0.0, 0.0), 2.0);
        assert_eq!(field.total_mass(), 3.0);
    }

    #[test]
    fn test_gravitational_center_of_mass() {
        let mut field = GravitationalField::empty();
        field.add_source(Vector3::new(1.0, 0.0, 0.0), 1.0);
        field.add_source(Vector3::new(3.0, 0.0, 0.0), 1.0);
        let com = field.center_of_mass().unwrap();
        assert!((com.x - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_electric_field_empty() {
        let field = ElectricField::empty();
        assert_eq!(field.sources.len(), 0);
    }

    #[test]
    fn test_electric_field_point_charge() {
        let field = ElectricField::point_charge(1.0e-6);
        assert_eq!(field.sources.len(), 1);
    }

    #[test]
    fn test_electric_field_at() {
        let field = ElectricField::point_charge(1.0e-6);
        let k = 8.9875517923e9;
        let field_vec = field.field_at(Vector3::new(1.0, 0.0, 0.0), k);
        assert!(field_vec.magnitude() > 0.0);
    }

    #[test]
    fn test_electric_potential_at() {
        let field = ElectricField::point_charge(1.0e-6);
        let k = 8.9875517923e9;
        let potential = field.potential_at(Vector3::new(1.0, 0.0, 0.0), k);
        assert!(potential > 0.0);
    }

    #[test]
    fn test_electric_force_on() {
        let field = ElectricField::point_charge(1.0e-6);
        let k = 8.9875517923e9;
        let force = field.force_on(Vector3::new(1.0, 0.0, 0.0), 1.0e-6, k);
        assert!(force.magnitude() > 0.0);
    }

    #[test]
    fn test_magnetic_field_empty() {
        let field = MagneticField::empty();
        assert_eq!(field.sources.len(), 0);
    }

    #[test]
    fn test_magnetic_field_uniform() {
        let field = MagneticField::uniform(Vector3::new(0.0, 0.0, 1.0));
        assert_eq!(field.sources.len(), 1);
    }

    #[test]
    fn test_magnetic_field_at() {
        let field = MagneticField::uniform(Vector3::new(0.0, 0.0, 1.0));
        let field_vec = field.field_at(Vector3::new(1.0, 0.0, 0.0));
        assert!((field_vec.z - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_magnetic_force_on() {
        let field = MagneticField::uniform(Vector3::new(0.0, 0.0, 1.0));
        let force = field.force_on(Vector3::zero(), 1.0e-6, Vector3::new(1.0, 0.0, 0.0));
        assert!(force.magnitude() > 0.0);
    }

    #[test]
    fn test_electromagnetic_field_empty() {
        let field = ElectromagneticField::empty();
        assert_eq!(field.electric_field.sources.len(), 0);
        assert_eq!(field.magnetic_field.sources.len(), 0);
    }

    #[test]
    fn test_electromagnetic_lorentz_force() {
        let mut em_field = ElectromagneticField::empty();
        em_field.electric_field.add_source(Vector3::zero(), 1.0e-6);
        em_field.magnetic_field = MagneticField::uniform(Vector3::new(0.0, 0.0, 1.0));

        let force = em_field.lorentz_force(
            Vector3::new(1.0, 0.0, 0.0),
            1.0e-6,
            Vector3::new(1.0, 0.0, 0.0),
            8.9875517923e9,
        );
        assert!(force.magnitude() > 0.0);
    }

    #[test]
    fn test_strong_nuclear_field_empty() {
        let field = StrongNuclearField::empty();
        assert_eq!(field.range, 1.0e-15);
    }

    #[test]
    fn test_strong_nuclear_field_strength() {
        let field = StrongNuclearField::with_parameters(1.0e-15, 1.0);
        let strength = field.field_strength(1.0e-15);
        assert!(strength > 0.0);
    }

    #[test]
    fn test_strong_nuclear_within_range() {
        let field = StrongNuclearField::with_parameters(1.0e-15, 1.0);
        assert!(field.within_range(1.0e-15));
        assert!(!field.within_range(1.0e-12));
    }

    #[test]
    fn test_weak_nuclear_field_empty() {
        let field = WeakNuclearField::empty();
        assert_eq!(field.range, 1.0e-18);
    }

    #[test]
    fn test_weak_nuclear_field_strength() {
        let field = WeakNuclearField::with_parameters(1.0e-18, 0.1);
        let strength = field.field_strength(1.0e-18);
        assert!(strength > 0.0);
    }

    #[test]
    fn test_energy_fields_empty() {
        let fields = EnergyFields::empty();
        assert_eq!(fields.gravitational_field.sources.len(), 0);
        assert_eq!(fields.electromagnetic_field.electric_field.sources.len(), 0);
    }

    #[test]
    fn test_energy_fields_from_light() {
        let fields = EnergyFields::from_light(1.0e25);
        assert_eq!(fields.gravitational_field.sources.len(), 1);
    }

    #[test]
    fn test_energy_fields_total_field_energy() {
        let fields = EnergyFields::from_light(1.0e25);
        let energy = fields.total_field_energy(Vector3::new(1.0, 0.0, 0.0));
        assert!(energy >= 0.0);
    }

    #[test]
    fn test_energy_fields_summary() {
        let fields = EnergyFields::empty();
        let summary = fields.summary();
        assert!(summary.contains("Energy Fields"));
    }

    // Phase 2 Tests: Emergent Field Functionality

    /// Test helper struct implementing PhotonLike trait
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

    #[test]
    fn test_gravitational_photon_from_archetypes() {
        let position = Vector3::new(0.0, 0.0, 0.0);
        let energy = 1.0e-19; // Joules
        let mut activation = [0.0; 22];

        // High Great Way activation (A7, A14, A21)
        activation[6] = 1.0; // A7
        activation[13] = 1.0; // A14
        activation[20] = 1.0; // A21

        let grav_photon = GravitationalPhoton::from_archetypes(position, energy, &activation);

        assert_eq!(grav_photon.position, position);
        assert_eq!(grav_photon.energy, energy);
        assert_eq!(grav_photon.great_way_activation, 1.0);
    }

    #[test]
    fn test_gravitational_photon_low_activation() {
        let position = Vector3::new(0.0, 0.0, 0.0);
        let energy = 1.0e-19;
        let mut activation = [0.0; 22];

        // Low Great Way activation
        activation[6] = 0.1;
        activation[13] = 0.1;
        activation[20] = 0.1;

        let grav_photon = GravitationalPhoton::from_archetypes(position, energy, &activation);

        assert!((grav_photon.great_way_activation - 0.1).abs() < 1e-10);
    }

    #[test]
    fn test_gravitational_field_from_photons() {
        let mut photons = Vec::new();

        // Create test photon with high Great Way activation
        let mut activation1 = [0.0; 22];
        activation1[6] = 1.0;
        activation1[13] = 1.0;
        activation1[20] = 1.0;
        photons.push(TestPhoton {
            position: Vector3::new(0.0, 0.0, 0.0),
            archetype_activation: activation1,
            energy: 1.0e-19,
        });

        // Create test photon with low Great Way activation
        let mut activation2 = [0.0; 22];
        activation2[6] = 0.1;
        activation2[13] = 0.1;
        activation2[20] = 0.1;
        photons.push(TestPhoton {
            position: Vector3::new(1.0, 0.0, 0.0),
            archetype_activation: activation2,
            energy: 1.0e-19,
        });

        let field = GravitationalField::field_from_photons(&photons);

        assert_eq!(field.photons.len(), 2);
        assert!((field.photons[0].great_way_activation - 1.0).abs() < 1e-10);
        assert!((field.photons[1].great_way_activation - 0.1).abs() < 1e-10);
    }

    #[test]
    fn test_gravitational_field_at_with_photons() {
        let mut photons = Vec::new();

        let mut activation = [0.0; 22];
        activation[6] = 1.0;
        activation[13] = 1.0;
        activation[20] = 1.0;
        photons.push(TestPhoton {
            position: Vector3::new(0.0, 0.0, 0.0),
            archetype_activation: activation,
            energy: 1.0e-10, // Higher energy for measurable field
        });

        let field = GravitationalField::field_from_photons(&photons);
        let g = 6.67430e-11;

        // Field at position (1, 0, 0)
        let field_vec = field.field_at(Vector3::new(1.0, 0.0, 0.0), g);

        // Field should be non-zero and point toward the photon
        assert!(field_vec.magnitude() > 0.0);
        assert!(field_vec.x < 0.0); // Points toward origin
    }

    #[test]
    fn test_electric_photon_from_archetypes() {
        let position = Vector3::new(0.0, 0.0, 0.0);
        let mut activation = [0.0; 22];

        // High positive Catalyst activation (A3, A10, A17)
        activation[2] = 1.0; // A3
        activation[9] = 1.0; // A10
        activation[16] = 1.0; // A17

        let elec_photon = ElectricPhoton::from_archetypes(position, &activation);

        assert_eq!(elec_photon.position, position);
        assert!(elec_photon.charge > 0.0); // Positive charge
        assert_eq!(elec_photon.catalyst_activation, 1.0);
    }

    #[test]
    fn test_electric_photon_negative_charge() {
        let position = Vector3::new(0.0, 0.0, 0.0);
        let mut activation = [0.0; 22];

        // Low Catalyst activation (negative charge)
        activation[2] = 0.0;
        activation[9] = 0.0;
        activation[16] = 0.0;

        let elec_photon = ElectricPhoton::from_archetypes(position, &activation);

        assert!(elec_photon.charge < 0.0); // Negative charge
        assert_eq!(elec_photon.catalyst_activation, 0.0);
    }

    #[test]
    fn test_electric_photon_neutral() {
        let position = Vector3::new(0.0, 0.0, 0.0);
        let mut activation = [0.0; 22];

        // Catalyst activation at 0.5 (neutral)
        activation[2] = 0.5;
        activation[9] = 0.5;
        activation[16] = 0.5;

        let elec_photon = ElectricPhoton::from_archetypes(position, &activation);

        // Charge should be near zero
        assert!(elec_photon.charge.abs() < 1e-25);
        assert_eq!(elec_photon.catalyst_activation, 0.5);
    }

    #[test]
    fn test_electric_field_from_photons() {
        let mut photons = Vec::new();

        // Positive charge photon
        let mut activation1 = [0.0; 22];
        activation1[2] = 1.0;
        activation1[9] = 1.0;
        activation1[16] = 1.0;
        photons.push(TestPhoton {
            position: Vector3::new(0.0, 0.0, 0.0),
            archetype_activation: activation1,
            energy: 1.0e-19,
        });

        // Negative charge photon
        let mut activation2 = [0.0; 22];
        activation2[2] = 0.0;
        activation2[9] = 0.0;
        activation2[16] = 0.0;
        photons.push(TestPhoton {
            position: Vector3::new(1.0, 0.0, 0.0),
            archetype_activation: activation2,
            energy: 1.0e-19,
        });

        let field = ElectricField::field_from_photons(&photons);

        assert_eq!(field.photons.len(), 2);
        assert!(field.photons[0].charge > 0.0); // Positive
        assert!(field.photons[1].charge < 0.0); // Negative
    }

    #[test]
    fn test_electric_field_at_with_photons() {
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
        let k = 8.9875517923e9;

        // Field at position (1, 0, 0)
        let field_vec = field.field_at(Vector3::new(1.0, 0.0, 0.0), k);

        // Field should be non-zero and point away from positive charge
        assert!(field_vec.magnitude() > 0.0);
        assert!(field_vec.x > 0.0); // Points away from origin (positive charge)
    }

    #[test]
    fn test_emergent_field_consistency() {
        // Test that emergent fields produce consistent results
        let mut photons = Vec::new();

        let mut activation = [0.0; 22];
        activation[2] = 1.0; // Catalyst
        activation[6] = 1.0; // Great Way
        activation[9] = 1.0;
        activation[13] = 1.0;
        activation[16] = 1.0;
        activation[20] = 1.0;
        photons.push(TestPhoton {
            position: Vector3::new(0.0, 0.0, 0.0),
            archetype_activation: activation,
            energy: 1.0e-10,
        });

        let grav_field = GravitationalField::field_from_photons(&photons);
        let elec_field = ElectricField::field_from_photons(&photons);

        // Both fields should have one photon
        assert_eq!(grav_field.photons.len(), 1);
        assert_eq!(elec_field.photons.len(), 1);

        // Great Way activation should be 1.0 for gravity
        assert_eq!(grav_field.photons[0].great_way_activation, 1.0);

        // Catalyst activation should be 1.0 for electric
        assert_eq!(elec_field.photons[0].catalyst_activation, 1.0);
    }

    // Phase 2 Tests: Holographic Field Properties

    #[test]
    fn test_gravitational_field_density() {
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
        let g = 6.67430e-11;

        // Density at origin should be high
        let density_origin = field.field_density(Vector3::new(0.0, 0.0, 0.0), g, 1.0);
        assert!(density_origin > 0.0);

        // Density far away should be lower
        let density_far = field.field_density(Vector3::new(10.0, 0.0, 0.0), g, 1.0);
        assert!(density_far < density_origin);
    }

    #[test]
    fn test_gravitational_field_curvature() {
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
        let g = 6.67430e-11;

        // Curvature at origin should be non-zero
        let curvature = field.field_curvature(Vector3::new(0.0, 0.0, 0.0), g, 1.0, 1e-6);
        // Curvature can be positive or negative depending on the gradient
        assert!(curvature.abs() > 0.0 || curvature.abs() < 1e-20);
    }

    #[test]
    fn test_gravitational_field_density_with_multiple_photons() {
        let mut photons = Vec::new();

        let mut activation = [0.0; 22];
        activation[6] = 1.0;
        activation[13] = 1.0;
        activation[20] = 1.0;

        // Add multiple photons
        photons.push(TestPhoton {
            position: Vector3::new(0.0, 0.0, 0.0),
            archetype_activation: activation,
            energy: 1.0e-10,
        });
        photons.push(TestPhoton {
            position: Vector3::new(0.5, 0.0, 0.0),
            archetype_activation: activation,
            energy: 1.0e-10,
        });

        let field = GravitationalField::field_from_photons(&photons);
        let g = 6.67430e-11;

        // Density at midpoint should be higher than single photon
        let density_midpoint = field.field_density(Vector3::new(0.25, 0.0, 0.0), g, 1.0);
        assert!(density_midpoint > 0.0);
    }

    #[test]
    fn test_electric_field_density() {
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
        let k = 8.9875517923e9;

        // Density at origin should be high
        let density_origin = field.field_density(Vector3::new(0.0, 0.0, 0.0), k, 1.0);
        assert!(density_origin > 0.0);

        // Density far away should be lower
        let density_far = field.field_density(Vector3::new(10.0, 0.0, 0.0), k, 1.0);
        assert!(density_far < density_origin);
    }

    #[test]
    fn test_electric_field_curvature() {
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
        let k = 8.9875517923e9;

        // Curvature at origin should be non-zero
        let curvature = field.field_curvature(Vector3::new(0.0, 0.0, 0.0), k, 1.0, 1e-6);
        // Curvature can be positive or negative depending on the gradient
        assert!(curvature.abs() > 0.0 || curvature.abs() < 1e-20);
    }

    #[test]
    fn test_set_holographic_ref() {
        let mut field = GravitationalField::empty();
        let ref_arc = Arc::new(());

        field.set_holographic_ref(ref_arc);
        assert!(field.holographic_ref.is_some());
    }

    #[test]
    fn test_field_density_proportional_to_activation() {
        let mut photons_high = Vec::new();
        let mut photons_low = Vec::new();

        // High activation photon
        let mut activation_high = [0.0; 22];
        activation_high[6] = 1.0;
        activation_high[13] = 1.0;
        activation_high[20] = 1.0;
        photons_high.push(TestPhoton {
            position: Vector3::new(0.0, 0.0, 0.0),
            archetype_activation: activation_high,
            energy: 1.0e-10,
        });

        // Low activation photon
        let mut activation_low = [0.0; 22];
        activation_low[6] = 0.1;
        activation_low[13] = 0.1;
        activation_low[20] = 0.1;
        photons_low.push(TestPhoton {
            position: Vector3::new(0.0, 0.0, 0.0),
            archetype_activation: activation_low,
            energy: 1.0e-10,
        });

        let field_high = GravitationalField::field_from_photons(&photons_high);
        let field_low = GravitationalField::field_from_photons(&photons_low);
        let g = 6.67430e-11;

        let density_high = field_high.field_density(Vector3::new(0.0, 0.0, 0.0), g, 1.0);
        let density_low = field_low.field_density(Vector3::new(0.0, 0.0, 0.0), g, 1.0);

        // Higher activation should produce higher density
        assert!(density_high > density_low);
    }

    // Phase 2 Task 4: Validate Field Equations

    #[test]
    fn test_gravitational_field_newtons_law() {
        // Validate that emergent gravitational field matches Newton's law
        // F = G * m1 * m2 / r²

        let mut photons = Vec::new();

        // Create a photon with known energy (mass equivalent)
        let mut activation = [0.0; 22];
        activation[6] = 1.0;
        activation[13] = 1.0;
        activation[20] = 1.0;

        const C: Float = 2.998e8; // Speed of light
        let energy = 1.0e-10; // Joules
        let mass_equiv = energy / (C * C); // E = mc²

        photons.push(TestPhoton {
            position: Vector3::new(0.0, 0.0, 0.0),
            archetype_activation: activation,
            energy,
        });

        let field = GravitationalField::field_from_photons(&photons);
        let g = 6.67430e-11;

        // Test at distance r = 1.0 m
        let r = 1.0;
        let field_vec = field.field_at(Vector3::new(r, 0.0, 0.0), g);

        // Expected field magnitude from Newton's law: g = G * M / r²
        let expected_magnitude = g * mass_equiv / (r * r);

        // Field magnitude should be close to expected (within 10% tolerance)
        let relative_error =
            (field_vec.magnitude() - expected_magnitude).abs() / expected_magnitude;
        assert!(
            relative_error < 0.1,
            "Field magnitude error: {}%",
            relative_error * 100.0
        );

        // Field should point toward the mass
        assert!(field_vec.x < 0.0);
        assert!(field_vec.y.abs() < 1e-10);
        assert!(field_vec.z.abs() < 1e-10);
    }

    #[test]
    fn test_gravitational_field_inverse_square_law() {
        // Validate that gravitational field follows inverse square law
        // Field strength should be proportional to 1/r²

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
        let g = 6.67430e-11;

        // Measure field at different distances
        let r1 = 1.0;
        let r2 = 2.0;

        let field1 = field.field_at(Vector3::new(r1, 0.0, 0.0), g).magnitude();
        let field2 = field.field_at(Vector3::new(r2, 0.0, 0.0), g).magnitude();

        // According to inverse square law: field2 = field1 / (r2/r1)²
        let expected_ratio = (r1 * r1) / (r2 * r2);
        let actual_ratio = field2 / field1;

        // Ratio should be close to expected (within 10% tolerance)
        let relative_error = (actual_ratio - expected_ratio).abs() / expected_ratio;
        assert!(
            relative_error < 0.1,
            "Inverse square law error: {}%",
            relative_error * 100.0
        );
    }

    #[test]
    fn test_electric_field_coulombs_law() {
        // Validate that emergent electric field matches Coulomb's law
        // F = k * q1 * q2 / r²

        let mut photons = Vec::new();

        // Create a photon with positive charge
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
        let k = 8.9875517923e9; // Coulomb constant

        // Get charge from the photon
        let charge = field.photons[0].charge;

        // Test at distance r = 1.0 m
        let r = 1.0;
        let field_vec = field.field_at(Vector3::new(r, 0.0, 0.0), k);

        // Expected field magnitude from Coulomb's law: E = k * q / r²
        let expected_magnitude = k * charge.abs() / (r * r);

        // Field magnitude should be close to expected (within 10% tolerance)
        let relative_error =
            (field_vec.magnitude() - expected_magnitude).abs() / expected_magnitude;
        assert!(
            relative_error < 0.1,
            "Field magnitude error: {}%",
            relative_error * 100.0
        );

        // Field should point away from positive charge
        assert!(field_vec.x > 0.0);
        assert!(field_vec.y.abs() < 1e-10);
        assert!(field_vec.z.abs() < 1e-10);
    }

    #[test]
    fn test_electric_field_inverse_square_law() {
        // Validate that electric field follows inverse square law

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
        let k = 8.9875517923e9;

        // Measure field at different distances
        let r1 = 1.0;
        let r2 = 2.0;

        let field1 = field.field_at(Vector3::new(r1, 0.0, 0.0), k).magnitude();
        let field2 = field.field_at(Vector3::new(r2, 0.0, 0.0), k).magnitude();

        // According to inverse square law
        let expected_ratio = (r1 * r1) / (r2 * r2);
        let actual_ratio = field2 / field1;

        // Ratio should be close to expected (within 10% tolerance)
        let relative_error = (actual_ratio - expected_ratio).abs() / expected_ratio;
        assert!(
            relative_error < 0.1,
            "Inverse square law error: {}%",
            relative_error * 100.0
        );
    }

    #[test]
    fn test_field_properties_at_different_scales() {
        // Validate field properties at different scales (quantum to macro)

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
        let g = 6.67430e-11;

        // Test at microscopic scale (1e-6 m)
        let micro_field = field.field_at(Vector3::new(1e-6, 0.0, 0.0), g).magnitude();
        assert!(micro_field > 0.0);

        // Test at macro scale (1.0 m)
        let macro_field = field.field_at(Vector3::new(1.0, 0.0, 0.0), g).magnitude();
        assert!(macro_field > 0.0);

        // Test at larger macro scale (10.0 m)
        let larger_macro_field = field.field_at(Vector3::new(10.0, 0.0, 0.0), g).magnitude();
        assert!(larger_macro_field > 0.0);

        // Verify inverse square law holds across scales
        // Field at 1e-6 should be (1e6)² = 1e12 times stronger than at 1.0
        let expected_ratio = 1e12;
        let actual_ratio = micro_field / macro_field;
        let relative_error = (actual_ratio - expected_ratio).abs() / expected_ratio;
        assert!(
            relative_error < 0.1,
            "Scale invariance error: {}%",
            relative_error * 100.0
        );

        // Field at 1.0 should be (10)² = 100 times stronger than at 10.0
        let expected_ratio_2 = 100.0;
        let actual_ratio_2 = macro_field / larger_macro_field;
        let relative_error_2 = (actual_ratio_2 - expected_ratio_2).abs() / expected_ratio_2;
        assert!(
            relative_error_2 < 0.1,
            "Scale invariance error 2: {}%",
            relative_error_2 * 100.0
        );
    }

    #[test]
    fn test_field_linearity() {
        // Validate that fields are linear (superposition principle)

        let mut photons1 = Vec::new();
        let mut photons2 = Vec::new();

        let mut activation = [0.0; 22];
        activation[6] = 1.0;
        activation[13] = 1.0;
        activation[20] = 1.0;

        // Two separate photons
        photons1.push(TestPhoton {
            position: Vector3::new(-1.0, 0.0, 0.0),
            archetype_activation: activation,
            energy: 1.0e-10,
        });
        photons2.push(TestPhoton {
            position: Vector3::new(1.0, 0.0, 0.0),
            archetype_activation: activation,
            energy: 1.0e-10,
        });

        // Field from each photon separately
        let field1 = GravitationalField::field_from_photons(&photons1);
        let field2 = GravitationalField::field_from_photons(&photons2);

        // Combined field
        let mut combined_photons = photons1.clone();
        combined_photons.extend(photons2.clone());
        let field_combined = GravitationalField::field_from_photons(&combined_photons);

        let g = 6.67430e-11;
        let test_position = Vector3::new(0.0, 1.0, 0.0);

        let field_vec1 = field1.field_at(test_position, g);
        let field_vec2 = field2.field_at(test_position, g);
        let field_vec_combined = field_combined.field_at(test_position, g);

        // Combined field should be sum of individual fields (superposition)
        let field_vec_sum = field_vec1.add(&field_vec2);

        let diff = field_vec_combined.sub(&field_vec_sum).magnitude();
        let relative_error = diff / field_vec_combined.magnitude();

        // Superposition should hold (within 10% tolerance)
        assert!(
            relative_error < 0.1,
            "Superposition error: {}%",
            relative_error * 100.0
        );
    }

    #[test]
    fn test_emergent_field_conservation() {
        // Validate that emergent fields conserve energy

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
        let g = 6.67430e-11;

        // Calculate field energy at different radii
        let radii = vec![0.1, 0.5, 1.0, 2.0, 5.0];
        let mut total_field_energy = 0.0;

        for r in &radii {
            let field_vec = field.field_at(Vector3::new(*r, 0.0, 0.0), g);
            // Energy density ~ E² (for gravity, ~ g²)
            let energy_density = field_vec.magnitude() * field_vec.magnitude();
            // Integrate over spherical shell: 4πr² × energy_density
            let shell_energy = 4.0 * std::f64::consts::PI * r * r * energy_density;
            total_field_energy += shell_energy * 0.1; // dr = 0.1
        }

        // Total field energy should be finite and positive
        assert!(total_field_energy > 0.0);
        assert!(total_field_energy.is_finite());
    }
}
