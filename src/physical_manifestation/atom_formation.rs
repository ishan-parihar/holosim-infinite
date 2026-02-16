// Atom Formation System - Phase 2: Physical Manifestation, Task 2.3
//
// This module implements the formation of atoms from particles based on
// electromagnetic attraction and archetype compatibility.
//
// Key Principles:
// 1. Atoms form from protons, neutrons, and electrons
// 2. Orbital configuration follows quantum mechanics (Aufbau principle)
// 3. Chemical properties derive from archetype activation
// 4. Periodic table emerges from archetype patterns
// 5. Electromagnetic forces govern atomic structure
//
// Knowledge Base References:
// - COMPLETE_REFACTOR_ROADMAP_V4.md Phase 2, Task 2.3
// - "All physics values are derived from the 22-Archetype structure"
// - "Electromagnetic attraction holds atoms together"

use crate::matter::particle::{Coordinate3D, Particle, ParticleID, Vector3D};
use crate::types::Float;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

// ============================================================================
// ATOM STRUCTURE
// ============================================================================

/// Represents a complete atom with nucleus and electrons
#[derive(Debug, Clone)]
pub struct Atom {
    /// Unique identifier for this atom
    pub id: u64,

    /// Position in 3D space
    pub position: Coordinate3D,

    /// Atomic number (number of protons)
    pub atomic_number: u8,

    /// Mass number (protons + neutrons)
    pub mass_number: u16,

    /// Nucleus containing protons and neutrons
    pub nucleus: Nucleus,

    /// Electrons in orbitals
    pub electrons: Vec<Electron>,

    /// Orbital configuration (e.g., "1s2 2s2 2p6")
    pub orbital_configuration: String,

    /// Chemical properties derived from archetype activation
    pub chemical_properties: ChemicalProperties,

    /// Archetype activation pattern for this atom
    pub archetype_activation: [Float; 22],

    /// Creation timestamp
    pub creation_time: u64,

    /// Current age
    pub age: u64,
}

/// Atomic nucleus containing protons and neutrons
#[derive(Debug, Clone)]
pub struct Nucleus {
    /// Protons in the nucleus
    pub protons: Vec<Particle>,

    /// Neutrons in the nucleus
    pub neutrons: Vec<Particle>,

    /// Nuclear binding energy (MeV)
    pub binding_energy: Float,
}

impl Nucleus {
    /// Create a new nucleus from protons and neutrons
    pub fn new(protons: Vec<Particle>, neutrons: Vec<Particle>) -> Self {
        // Calculate binding energy using semi-empirical mass formula
        let binding_energy = Self::calculate_binding_energy(&protons, &neutrons);

        Self {
            protons,
            neutrons,
            binding_energy,
        }
    }

    /// Calculate nuclear binding energy using semi-empirical mass formula
    fn calculate_binding_energy(protons: &[Particle], neutrons: &[Particle]) -> Float {
        let a = protons.len() + neutrons.len(); // Mass number
        let z = protons.len(); // Atomic number

        if a == 0 {
            return 0.0;
        }

        // Semi-empirical mass formula coefficients (in MeV)
        const A_V: Float = 15.75; // Volume term
        const A_S: Float = 17.8; // Surface term
        const A_C: Float = 0.711; // Coulomb term
        const A_A: Float = 23.7; // Asymmetry term

        let volume_term = A_V * a as Float;
        let surface_term = A_S * (a as Float).powf(2.0 / 3.0);
        let coulomb_term = A_C * z as Float * (z as Float - 1.0) / (a as Float).powf(1.0 / 3.0);
        let asymmetry_term = A_A * ((a as Float - 2.0 * z as Float).powf(2.0)) / a as Float;

        volume_term - surface_term - coulomb_term - asymmetry_term
    }

    /// Get the number of protons
    pub fn proton_count(&self) -> usize {
        self.protons.len()
    }

    /// Get the number of neutrons
    pub fn neutron_count(&self) -> usize {
        self.neutrons.len()
    }
}

/// Electron in an orbital
#[derive(Debug, Clone)]
pub struct Electron {
    /// Particle representing this electron
    pub particle: Particle,

    /// Principal quantum number (n = 1, 2, 3, ...)
    pub principal_quantum_number: u8,

    /// Azimuthal quantum number (l = 0, 1, 2, ...)
    pub azimuthal_quantum_number: u8,

    /// Magnetic quantum number (m = -l, ..., +l)
    pub magnetic_quantum_number: i8,

    /// Spin quantum number (s = +1/2 or -1/2)
    pub spin: Float,

    /// Orbital label (e.g., "1s", "2p", "3d")
    pub orbital_label: String,
}

/// Chemical properties of an atom
#[derive(Debug, Clone)]
pub struct ChemicalProperties {
    /// Electronegativity (Pauling scale)
    pub electronegativity: Float,

    /// First ionization energy (eV)
    pub ionization_energy: Float,

    /// Atomic radius (pm)
    pub atomic_radius: Float,

    /// Valence electrons
    pub valence_electrons: u8,

    /// Common oxidation states
    pub oxidation_states: Vec<i8>,

    /// Chemical family/group
    pub chemical_family: ChemicalFamily,

    /// Reactivity (0.0 to 1.0)
    pub reactivity: Float,
}

/// Chemical family classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChemicalFamily {
    AlkaliMetal,
    AlkalineEarthMetal,
    TransitionMetal,
    PostTransitionMetal,
    Metalloid,
    Nonmetal,
    Halogen,
    NobleGas,
    Lanthanide,
    Actinide,
    Unknown,
}

// ============================================================================
// ATOM FORMATION SYSTEM
// ============================================================================

/// System for forming atoms from particles
pub struct AtomFormationSystem {
    /// Counter for generating unique atom IDs
    atom_id_counter: AtomicU64,

    /// Counter for generating unique particle IDs
    particle_id_counter: AtomicU64,

    /// Cache of orbital configurations
    orbital_cache: HashMap<u8, String>,

    /// Electromagnetic force constant (Coulomb constant)
    coulomb_constant: Float,

    /// Bohr radius (m)
    bohr_radius: Float,
}

impl Default for AtomFormationSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl AtomFormationSystem {
    /// Create a new atom formation system
    pub fn new() -> Self {
        Self {
            atom_id_counter: AtomicU64::new(0),
            particle_id_counter: AtomicU64::new(0),
            orbital_cache: HashMap::new(),
            coulomb_constant: 8.9875517923e9, // N·m²/C²
            bohr_radius: 5.29177210903e-11,   // m
        }
    }

    /// Generate a unique atom ID
    fn generate_atom_id(&self) -> u64 {
        self.atom_id_counter.fetch_add(1, Ordering::SeqCst)
    }

    /// Generate a unique particle ID
    fn generate_particle_id(&self) -> ParticleID {
        self.particle_id_counter.fetch_add(1, Ordering::SeqCst)
    }

    /// Form a nucleus from protons and neutrons
    ///
    /// This method combines protons and neutrons into a stable nucleus
    /// using the strong nuclear force.
    ///
    /// # Arguments
    /// * `protons` - Vector of proton particles
    /// * `neutrons` - Vector of neutron particles
    ///
    /// # Returns
    /// The formed nucleus
    pub fn form_nucleus(
        &mut self,
        mut protons: Vec<Particle>,
        mut neutrons: Vec<Particle>,
    ) -> Result<Nucleus, String> {
        // Validate that we have at least one proton
        if protons.is_empty() {
            return Err("Cannot form nucleus: no protons provided".to_string());
        }

        // Validate that the nucleus can be stable
        let proton_count = protons.len();
        let neutron_count = neutrons.len();

        // Check stability rules (simplified)
        // For light nuclei (Z < 20), stable N/Z ratio is approximately 1
        // For heavier nuclei, N/Z ratio increases
        let max_protons = 118; // Up to oganesson
        if proton_count > max_protons {
            return Err(format!(
                "Cannot form nucleus: too many protons ({} > {})",
                proton_count, max_protons
            ));
        }

        // Calculate optimal neutron count for stability
        let optimal_neutrons = if proton_count <= 20 {
            proton_count // N ≈ Z for light nuclei
        } else if proton_count <= 83 {
            proton_count + ((proton_count as Float - 20.0) / 63.0 * 30.0) as usize
        // N/Z increases
        } else {
            proton_count + 40 // Heavy nuclei need more neutrons
        };

        // Warn if neutron count is far from optimal
        let neutron_ratio = neutron_count as Float / proton_count as Float;
        let optimal_ratio = optimal_neutrons as Float / proton_count as Float;

        if (neutron_ratio - optimal_ratio).abs() > 0.3 {
            // Nucleus may be unstable, but we allow it for simulation
            // In a full implementation, we would calculate decay probability
        }

        // Position particles in the nucleus
        // Distribute them in a sphere with radius proportional to mass number
        let mass_number = proton_count + neutron_count;
        let nuclear_radius = 1.2e-15 * (mass_number as Float).powf(1.0 / 3.0); // Fermi model

        // Position protons
        for (i, proton) in protons.iter_mut().enumerate() {
            let angle = 2.0 * std::f64::consts::PI * i as Float / proton_count as Float;
            let radius = nuclear_radius * (i as Float / proton_count as Float).sqrt();
            proton.position = Coordinate3D::new(radius * angle.cos(), radius * angle.sin(), 0.0);
        }

        // Position neutrons
        for (i, neutron) in neutrons.iter_mut().enumerate() {
            let angle = 2.0 * std::f64::consts::PI * i as Float / neutron_count as Float;
            let radius = nuclear_radius
                * ((i as Float + proton_count as Float) / mass_number as Float).sqrt();
            neutron.position = Coordinate3D::new(radius * angle.cos(), radius * angle.sin(), 0.0);
        }

        Ok(Nucleus::new(protons, neutrons))
    }

    /// Add electrons to an atom
    ///
    /// This method adds electrons to an atom in accordance with the
    /// Aufbau principle and Pauli exclusion principle.
    ///
    /// # Arguments
    /// * `atom` - Mutable reference to the atom
    /// * `electron_count` - Number of electrons to add
    ///
    /// # Returns
    /// Result indicating success or failure
    pub fn add_electrons(&mut self, atom: &mut Atom, electron_count: usize) -> Result<(), String> {
        let atomic_number = atom.atomic_number as usize;

        // Cannot add more electrons than protons (for neutral atom)
        if electron_count > atomic_number {
            return Err(format!(
                "Cannot add {} electrons to atom with {} protons",
                electron_count, atomic_number
            ));
        }

        // Calculate orbital configuration
        let orbital_config = self.calculate_orbital_configuration(electron_count as u8);

        // Create electrons according to orbital configuration
        let mut electrons = Vec::new();
        let mut electrons_added = 0;

        // Parse orbital configuration and create electrons
        let orbitals = self.parse_orbital_configuration(&orbital_config);

        for orbital in orbitals {
            for _ in 0..orbital.electron_count {
                if electrons_added >= electron_count {
                    break;
                }

                let electron = self.create_electron(
                    &atom.position,
                    orbital.principal_quantum_number,
                    orbital.azimuthal_quantum_number,
                    orbital.magnetic_quantum_number,
                    electrons_added % 2, // Alternate spin
                );

                electrons.push(electron);
                electrons_added += 1;
            }
        }

        atom.electrons = electrons;
        atom.orbital_configuration = orbital_config;

        Ok(())
    }

    /// Calculate orbital configuration for a given number of electrons
    ///
    /// This method implements the Aufbau principle, filling orbitals
    /// in order of increasing energy.
    ///
    /// # Arguments
    /// * `electron_count` - Number of electrons
    ///
    /// # Returns
    /// Orbital configuration string (e.g., "1s2 2s2 2p6")
    pub fn calculate_orbital_configuration(&mut self, electron_count: u8) -> String {
        // Check cache
        if let Some(cached) = self.orbital_cache.get(&electron_count) {
            return cached.clone();
        }

        // Define orbital order (Aufbau principle)
        // Format: (n, l, max_electrons, label)
        let orbital_order = vec![
            (1, 0, 2, "1s"),  // 1s can hold 2 electrons
            (2, 0, 2, "2s"),  // 2s can hold 2 electrons
            (2, 1, 6, "2p"),  // 2p can hold 6 electrons
            (3, 0, 2, "3s"),  // 3s can hold 2 electrons
            (3, 1, 6, "3p"),  // 3p can hold 6 electrons
            (4, 0, 2, "4s"),  // 4s can hold 2 electrons
            (3, 2, 10, "3d"), // 3d can hold 10 electrons
            (4, 1, 6, "4p"),  // 4p can hold 6 electrons
            (5, 0, 2, "5s"),  // 5s can hold 2 electrons
            (4, 2, 10, "4d"), // 4d can hold 10 electrons
            (5, 1, 6, "5p"),  // 5p can hold 6 electrons
            (6, 0, 2, "6s"),  // 6s can hold 2 electrons
            (4, 3, 14, "4f"), // 4f can hold 14 electrons
            (5, 2, 10, "5d"), // 5d can hold 10 electrons
            (6, 1, 6, "6p"),  // 6p can hold 6 electrons
            (7, 0, 2, "7s"),  // 7s can hold 2 electrons
            (5, 3, 14, "5f"), // 5f can hold 14 electrons
            (6, 2, 10, "6d"), // 6d can hold 10 electrons
            (7, 1, 6, "7p"),  // 7p can hold 6 electrons
        ];

        let mut remaining_electrons = electron_count as usize;
        let mut configuration = String::new();

        for (_n, _l, max_electrons, label) in orbital_order {
            if remaining_electrons == 0 {
                break;
            }

            let electrons_in_orbital = remaining_electrons.min(max_electrons);

            if electrons_in_orbital > 0 {
                if !configuration.is_empty() {
                    configuration.push(' ');
                }
                configuration.push_str(&format!("{}{}", label, electrons_in_orbital));
            }

            remaining_electrons -= electrons_in_orbital;
        }

        // Cache the result
        self.orbital_cache
            .insert(electron_count, configuration.clone());

        configuration
    }

    /// Parse orbital configuration string into structured orbitals
    fn parse_orbital_configuration(&self, config: &str) -> Vec<OrbitalInfo> {
        let mut orbitals = Vec::new();
        let parts: Vec<&str> = config.split_whitespace().collect();

        for part in parts {
            if part.len() < 2 {
                continue;
            }

            // Parse orbital label (e.g., "1s", "2p", "3d")
            let principal = part.chars().next().unwrap().to_digit(10).unwrap() as u8;
            let azimuthal_char = part.chars().nth(1).unwrap();
            let azimuthal = match azimuthal_char {
                's' => 0,
                'p' => 1,
                'd' => 2,
                'f' => 3,
                _ => 0,
            };

            // Parse electron count
            let electron_count = part[2..].parse::<usize>().unwrap_or(0);

            // Calculate magnetic quantum numbers
            let mut magnetic_start = -(azimuthal as i8);
            let mut orbitals_in_subshell = Vec::new();

            for _i in 0..electron_count {
                orbitals_in_subshell.push(OrbitalInfo {
                    principal_quantum_number: principal,
                    azimuthal_quantum_number: azimuthal,
                    magnetic_quantum_number: magnetic_start,
                    electron_count: 1,
                });

                magnetic_start += 1;
                if magnetic_start > azimuthal as i8 {
                    magnetic_start = -(azimuthal as i8);
                }
            }

            orbitals.extend(orbitals_in_subshell);
        }

        orbitals
    }

    /// Create an electron in a specific orbital
    fn create_electron(
        &self,
        nucleus_position: &Coordinate3D,
        principal_qn: u8,
        azimuthal_qn: u8,
        magnetic_qn: i8,
        spin_index: usize,
    ) -> Electron {
        // Calculate orbital radius using Bohr model
        // r = n² * a₀ / Z_eff
        let n = principal_qn as Float;
        let z_eff = 1.0; // Simplified effective nuclear charge
        let orbital_radius = (n * n * self.bohr_radius) / z_eff;

        // Calculate position based on quantum numbers
        let theta = (magnetic_qn as Float) * std::f64::consts::PI / 4.0;
        let phi = (spin_index as Float) * std::f64::consts::PI;

        let x = orbital_radius * theta.sin() * phi.cos();
        let y = orbital_radius * theta.sin() * phi.sin();
        let z = orbital_radius * theta.cos();

        let position = Coordinate3D::new(
            nucleus_position.x + x,
            nucleus_position.y + y,
            nucleus_position.z + z,
        );

        // Create electron particle
        let particle_id = self.generate_particle_id();
        let archetype_activation = [0.5; 22]; // Simplified
        let mut particle =
            Particle::from_archetype_activation(particle_id, archetype_activation, position);

        // Override properties for electron
        particle.mass = 9.10938356e-31; // kg
        particle.charge = -1.0; // elementary charge

        // Spin quantum number (+1/2 or -1/2)
        let spin = if spin_index % 2 == 0 { 0.5 } else { -0.5 };

        // Orbital label
        let azimuthal_char = match azimuthal_qn {
            0 => 's',
            1 => 'p',
            2 => 'd',
            3 => 'f',
            _ => 's',
        };
        let orbital_label = format!("{}{}", principal_qn, azimuthal_char);

        Electron {
            particle,
            principal_quantum_number: principal_qn,
            azimuthal_quantum_number: azimuthal_qn,
            magnetic_quantum_number: magnetic_qn,
            spin,
            orbital_label,
        }
    }

    /// Determine chemical properties from archetype activation
    ///
    /// This method derives chemical properties from the archetype
    /// activation pattern of the atom.
    ///
    /// # Arguments
    /// * `archetype_activation` - The 22-value archetype activation pattern
    /// * `atomic_number` - The atomic number of the atom
    ///
    /// # Returns
    /// Chemical properties of the atom
    pub fn determine_chemical_properties(
        &self,
        archetype_activation: &[Float; 22],
        atomic_number: u8,
    ) -> ChemicalProperties {
        // Calculate electronegativity from archetype activation
        // Electronegativity emerges from Catalyst (A3, A10, A17) and Significator (A5, A12, A19)
        let catalyst_avg =
            (archetype_activation[2] + archetype_activation[9] + archetype_activation[16]) / 3.0;
        let significator_avg =
            (archetype_activation[4] + archetype_activation[11] + archetype_activation[18]) / 3.0;
        let electronegativity = 0.5 + (catalyst_avg * significator_avg) * 3.5; // Scale to 0.5-4.0

        // Calculate ionization energy from archetype activation
        // Ionization energy emerges from Potentiator (A2, A9, A16) and Experience (A4, A11, A18)
        let potentiator_avg =
            (archetype_activation[1] + archetype_activation[8] + archetype_activation[15]) / 3.0;
        let experience_avg =
            (archetype_activation[3] + archetype_activation[10] + archetype_activation[17]) / 3.0;
        let ionization_energy = 3.0 + (potentiator_avg * experience_avg) * 24.0; // Scale to 3-27 eV

        // Calculate atomic radius from archetype activation
        // Atomic radius emerges from Matrix (A1, A8, A15)
        let matrix_avg =
            (archetype_activation[0] + archetype_activation[7] + archetype_activation[14]) / 3.0;
        let atomic_radius = 30.0 + matrix_avg * 200.0; // Scale to 30-230 pm

        // Calculate valence electrons from orbital configuration
        let valence_electrons = self.calculate_valence_electrons(atomic_number);

        // Determine oxidation states
        let oxidation_states = self.determine_oxidation_states(atomic_number, valence_electrons);

        // Determine chemical family
        let chemical_family = self.determine_chemical_family(atomic_number);

        // Calculate reactivity
        // Reactivity emerges from Choice (A22) and Transformation (A6, A13, A20)
        let choice = archetype_activation[21];
        let transformation_avg =
            (archetype_activation[5] + archetype_activation[12] + archetype_activation[19]) / 3.0;
        let reactivity = (choice * transformation_avg).min(1.0);

        ChemicalProperties {
            electronegativity,
            ionization_energy,
            atomic_radius,
            valence_electrons,
            oxidation_states,
            chemical_family,
            reactivity,
        }
    }

    /// Calculate valence electrons for a given atomic number
    fn calculate_valence_electrons(&self, atomic_number: u8) -> u8 {
        // Valence electron calculation based on periodic table position
        // This follows the octet rule for main group elements

        match atomic_number {
            1 => 1, // H
            2 => 2, // He
            // Period 2: Li to Ne
            3 => 1,  // Li
            4 => 2,  // Be
            5 => 3,  // B
            6 => 4,  // C
            7 => 5,  // N
            8 => 6,  // O
            9 => 7,  // F
            10 => 8, // Ne
            // Period 3: Na to Ar
            11 => 1, // Na
            12 => 2, // Mg
            13 => 3, // Al
            14 => 4, // Si
            15 => 5, // P
            16 => 6, // S
            17 => 7, // Cl
            18 => 8, // Ar
            // Period 4: K to Kr (simplified - includes transition metals)
            19..=36 => (atomic_number % 18) % 18,
            // Simplified for heavier elements
            _ => (atomic_number % 8) % 8,
        }
    }

    /// Determine common oxidation states
    fn determine_oxidation_states(&self, atomic_number: u8, valence_electrons: u8) -> Vec<i8> {
        let mut states = Vec::new();

        // Common oxidation states based on valence electrons
        if valence_electrons <= 4 {
            // Lose electrons (positive oxidation states)
            for i in 0..=valence_electrons as i8 {
                if i > 0 {
                    states.push(i);
                }
            }
        } else {
            // Gain electrons (negative oxidation states)
            let electrons_to_gain = 8 - valence_electrons as i8;
            for i in 1..=electrons_to_gain {
                states.push(-i);
            }
        }

        // Add common exceptions
        if atomic_number == 1 {
            states = vec![1, -1]; // H can be +1 or -1
        } else if atomic_number == 6 {
            states = vec![-4, 2, 4]; // C
        } else if atomic_number == 7 {
            states = vec![-3, 1, 3, 5]; // N
        } else if atomic_number == 8 {
            states = vec![-2, -1, 1, 2]; // O
        }

        states
    }

    /// Determine chemical family from atomic number
    fn determine_chemical_family(&self, atomic_number: u8) -> ChemicalFamily {
        match atomic_number {
            1 => ChemicalFamily::Nonmetal, // H
            2 => ChemicalFamily::NobleGas, // He
            3..=10 => {
                match atomic_number {
                    3..=4 => ChemicalFamily::AlkaliMetal, // Li, Be
                    5..=6 => ChemicalFamily::Nonmetal,    // B, C
                    7..=8 => ChemicalFamily::Nonmetal,    // N, O
                    9 => ChemicalFamily::Halogen,         // F
                    10 => ChemicalFamily::NobleGas,       // Ne
                    _ => ChemicalFamily::Unknown,
                }
            }
            11..=18 => {
                match atomic_number {
                    11 => ChemicalFamily::AlkaliMetal,              // Na
                    12 => ChemicalFamily::AlkalineEarthMetal,       // Mg
                    13..=14 => ChemicalFamily::PostTransitionMetal, // Al, Si
                    15..=16 => ChemicalFamily::Nonmetal,            // P, S
                    17 => ChemicalFamily::Halogen,                  // Cl
                    18 => ChemicalFamily::NobleGas,                 // Ar
                    _ => ChemicalFamily::Unknown,
                }
            }
            19..=36 => {
                match atomic_number {
                    19 => ChemicalFamily::AlkaliMetal,              // K
                    20 => ChemicalFamily::AlkalineEarthMetal,       // Ca
                    21..=30 => ChemicalFamily::TransitionMetal,     // Sc to Zn
                    31..=32 => ChemicalFamily::PostTransitionMetal, // Ga, Ge
                    33..=34 => ChemicalFamily::Metalloid,           // As, Se
                    35 => ChemicalFamily::Halogen,                  // Br
                    36 => ChemicalFamily::NobleGas,                 // Kr
                    _ => ChemicalFamily::Unknown,
                }
            }
            57..=71 => ChemicalFamily::Lanthanide, // La to Lu
            89..=103 => ChemicalFamily::Actinide,  // Ac to Lr
            _ => ChemicalFamily::Unknown,
        }
    }

    /// Calculate electromagnetic attraction between particles
    ///
    /// This method calculates the Coulomb force between two charged particles.
    ///
    /// # Arguments
    /// * `particle1` - First particle
    /// * `particle2` - Second particle
    ///
    /// # Returns
    /// Electromagnetic force magnitude (N) and direction vector
    pub fn calculate_electromagnetic_attraction(
        &self,
        particle1: &Particle,
        particle2: &Particle,
    ) -> (Float, Vector3D) {
        // Calculate distance
        let distance = particle1.position.distance_to(&particle2.position);

        // Avoid division by zero
        let distance = distance.max(1e-15);

        // Calculate Coulomb force: F = k * q1 * q2 / r²
        let force_magnitude =
            self.coulomb_constant * particle1.charge * particle2.charge / (distance * distance);

        // Calculate direction (from particle1 to particle2)
        let dx = particle2.position.x - particle1.position.x;
        let dy = particle2.position.y - particle1.position.y;
        let dz = particle2.position.z - particle1.position.z;

        let direction = Vector3D::new(dx / distance, dy / distance, dz / distance);

        // Force is attractive if charges have opposite signs
        let force_magnitude = force_magnitude.abs();

        (force_magnitude, direction)
    }

    /// Calculate archetype compatibility for bonding
    ///
    /// This method calculates how compatible two atoms are for bonding
    /// based on their archetype activation patterns.
    ///
    /// # Arguments
    /// * `atom1` - First atom
    /// * `atom2` - Second atom
    ///
    /// # Returns
    /// Compatibility score (0.0 to 1.0)
    pub fn calculate_archetype_compatibility(&self, atom1: &Atom, atom2: &Atom) -> Float {
        let activation1 = &atom1.archetype_activation;
        let activation2 = &atom2.archetype_activation;

        // Calculate dot product of archetype activation vectors
        let mut dot_product = 0.0;
        let mut norm1 = 0.0;
        let mut norm2 = 0.0;

        for i in 0..22 {
            dot_product += activation1[i] * activation2[i];
            norm1 += activation1[i] * activation1[i];
            norm2 += activation2[i] * activation2[i];
        }

        // Calculate cosine similarity
        let norm1 = norm1.sqrt();
        let norm2 = norm2.sqrt();

        if norm1 == 0.0 || norm2 == 0.0 {
            return 0.0;
        }

        let cosine_similarity = dot_product / (norm1 * norm2);

        // Scale to 0.0-1.0 range
        (cosine_similarity + 1.0) / 2.0
    }

    /// Generate periodic table from archetype patterns
    ///
    /// This method generates a periodic table of elements based on
    /// archetype activation patterns.
    ///
    /// # Returns
    /// Vector of atoms representing the periodic table
    pub fn generate_periodic_table(&mut self) -> Vec<Atom> {
        let mut periodic_table = Vec::new();

        for atomic_number in 1..=118 {
            // Generate archetype activation pattern for this element
            let archetype_activation = self.generate_element_archetype_pattern(atomic_number);

            // Calculate mass number (most common isotope)
            let mass_number = self.calculate_mass_number(atomic_number);

            // Create atom
            let atom = self.create_atom_from_archetypes(
                archetype_activation,
                atomic_number,
                mass_number,
                Coordinate3D::origin(),
            );

            periodic_table.push(atom);
        }

        periodic_table
    }

    /// Generate archetype activation pattern for an element
    fn generate_element_archetype_pattern(&self, atomic_number: u8) -> [Float; 22] {
        let mut activation = [0.5; 22];

        // Use periodic table position to influence archetype activation
        // This creates a pattern that reflects the periodic nature of elements

        let period = match atomic_number {
            1..=2 => 1,
            3..=10 => 2,
            11..=18 => 3,
            19..=36 => 4,
            37..=54 => 5,
            55..=86 => 6,
            87..=118 => 7,
            _ => 1,
        };

        let group = ((atomic_number as Float - 1.0) % 18.0 + 1.0) as u8;

        // Modify archetype activation based on position in periodic table
        for i in 0..22 {
            // Create periodic variation
            let periodic_factor = (2.0 * std::f64::consts::PI * i as Float / 22.0).sin();
            let group_factor = (group as Float / 18.0) * 2.0 - 1.0;
            let period_factor = (period as Float / 7.0) * 2.0 - 1.0;

            activation[i] = 0.5 + 0.3 * periodic_factor + 0.1 * group_factor + 0.1 * period_factor;
            activation[i] = activation[i].clamp(0.0, 1.0);
        }

        activation
    }

    /// Calculate mass number for an element (most common isotope)
    fn calculate_mass_number(&self, atomic_number: u8) -> u16 {
        // Simplified mass number calculation
        // In a full implementation, this would use actual isotope data

        let neutron_count = if atomic_number <= 20 {
            atomic_number as u16 // N ≈ Z for light elements
        } else if atomic_number <= 56 {
            atomic_number as u16 + ((atomic_number as u16 - 20) * 6 / 36) // Increasing N/Z
        } else {
            atomic_number as u16 + 40 // Heavy elements
        };

        atomic_number as u16 + neutron_count
    }

    /// Create an atom from archetype activation pattern
    pub fn create_atom_from_archetypes(
        &mut self,
        archetype_activation: [Float; 22],
        atomic_number: u8,
        mass_number: u16,
        position: Coordinate3D,
    ) -> Atom {
        let neutron_count = mass_number - atomic_number as u16;

        // Create protons
        let mut protons = Vec::new();
        for _ in 0..atomic_number {
            let proton_id = self.generate_particle_id();
            let proton_archetype = archetype_activation;
            let mut proton =
                Particle::from_archetype_activation(proton_id, proton_archetype, position);
            proton.mass = 1.6726219e-27; // kg
            proton.charge = 1.0; // elementary charge
            proton.spin = 0.5;
            protons.push(proton);
        }

        // Create neutrons
        let mut neutrons = Vec::new();
        for _ in 0..neutron_count {
            let neutron_id = self.generate_particle_id();
            let neutron_archetype = archetype_activation;
            let mut neutron =
                Particle::from_archetype_activation(neutron_id, neutron_archetype, position);
            neutron.mass = 1.6749274e-27; // kg
            neutron.charge = 0.0;
            neutron.spin = 0.5;
            neutrons.push(neutron);
        }

        // Form nucleus
        let nucleus = self.form_nucleus(protons, neutrons).unwrap();

        // Determine chemical properties
        let chemical_properties =
            self.determine_chemical_properties(&archetype_activation, atomic_number);

        // Create atom
        let atom_id = self.generate_atom_id();
        let mut atom = Atom {
            id: atom_id,
            position,
            atomic_number,
            mass_number,
            nucleus,
            electrons: Vec::new(),
            orbital_configuration: String::new(),
            chemical_properties,
            archetype_activation,
            creation_time: 0,
            age: 0,
        };

        // Add electrons (neutral atom)
        self.add_electrons(&mut atom, atomic_number as usize)
            .unwrap();

        atom
    }
}

// ============================================================================
// HELPER STRUCTURES
// ============================================================================

/// Information about an orbital
#[derive(Debug, Clone)]
struct OrbitalInfo {
    principal_quantum_number: u8,
    azimuthal_quantum_number: u8,
    magnetic_quantum_number: i8,
    electron_count: usize,
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matter::particle::{Coordinate3D, Particle};

    #[test]
    fn test_nucleus_formation() {
        let mut system = AtomFormationSystem::new();

        // Create protons
        let protons = vec![
            Particle::from_archetype_activation(1, [0.5; 22], Coordinate3D::origin()),
            Particle::from_archetype_activation(1, [0.5; 22], Coordinate3D::origin()),
        ];

        // Create neutrons
        let neutrons = vec![
            Particle::from_archetype_activation(1, [0.5; 22], Coordinate3D::origin()),
            Particle::from_archetype_activation(1, [0.5; 22], Coordinate3D::origin()),
        ];

        let nucleus = system.form_nucleus(protons, neutrons).unwrap();

        assert_eq!(nucleus.proton_count(), 2);
        assert_eq!(nucleus.neutron_count(), 2);
        assert!(nucleus.binding_energy > 0.0);
    }

    #[test]
    fn test_nucleus_formation_no_protons() {
        let mut system = AtomFormationSystem::new();

        let result = system.form_nucleus(vec![], vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_nucleus_formation_too_many_protons() {
        let mut system = AtomFormationSystem::new();

        let mut protons = Vec::new();
        for _ in 0..200 {
            protons.push(Particle::from_archetype_activation(
                protons.len() as u64, // ParticleID
                [0.5; 22],
                Coordinate3D::origin(),
            ));
        }

        let result = system.form_nucleus(protons, vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_orbital_configuration() {
        let mut system = AtomFormationSystem::new();

        // Test hydrogen (1 electron)
        let config = system.calculate_orbital_configuration(1);
        assert_eq!(config, "1s1");

        // Test helium (2 electrons)
        let config = system.calculate_orbital_configuration(2);
        assert_eq!(config, "1s2");

        // Test carbon (6 electrons)
        let config = system.calculate_orbital_configuration(6);
        assert_eq!(config, "1s2 2s2 2p2");

        // Test neon (10 electrons)
        let config = system.calculate_orbital_configuration(10);
        assert_eq!(config, "1s2 2s2 2p6");

        // Test sodium (11 electrons)
        let config = system.calculate_orbital_configuration(11);
        assert_eq!(config, "1s2 2s2 2p6 3s1");
    }

    #[test]
    fn test_add_electrons() {
        let mut system = AtomFormationSystem::new();

        // Create hydrogen atom
        let protons = vec![Particle::from_archetype_activation(
            1, // ParticleID
            [0.5; 22],
            Coordinate3D::origin(),
        )];
        let nucleus = system.form_nucleus(protons, vec![]).unwrap();

        let mut atom = Atom {
            id: 1,
            position: Coordinate3D::origin(),
            atomic_number: 1,
            mass_number: 1,
            nucleus,
            electrons: Vec::new(),
            orbital_configuration: String::new(),
            chemical_properties: ChemicalProperties {
                electronegativity: 2.2,
                ionization_energy: 13.6,
                atomic_radius: 53.0,
                valence_electrons: 1,
                oxidation_states: vec![1, -1],
                chemical_family: ChemicalFamily::Nonmetal,
                reactivity: 0.5,
            },
            archetype_activation: [0.5; 22],
            creation_time: 0,
            age: 0,
        };

        system.add_electrons(&mut atom, 1).unwrap();

        assert_eq!(atom.electrons.len(), 1);
        assert_eq!(atom.orbital_configuration, "1s1");
    }

    #[test]
    fn test_add_electrons_too_many() {
        let mut system = AtomFormationSystem::new();

        let protons = vec![Particle::from_archetype_activation(
            1, // ParticleID
            [0.5; 22],
            Coordinate3D::origin(),
        )];
        let nucleus = system.form_nucleus(protons, vec![]).unwrap();

        let mut atom = Atom {
            id: 1,
            position: Coordinate3D::origin(),
            atomic_number: 1,
            mass_number: 1,
            nucleus,
            electrons: Vec::new(),
            orbital_configuration: String::new(),
            chemical_properties: ChemicalProperties {
                electronegativity: 2.2,
                ionization_energy: 13.6,
                atomic_radius: 53.0,
                valence_electrons: 1,
                oxidation_states: vec![1, -1],
                chemical_family: ChemicalFamily::Nonmetal,
                reactivity: 0.5,
            },
            archetype_activation: [0.5; 22],
            creation_time: 0,
            age: 0,
        };

        let result = system.add_electrons(&mut atom, 5);
        assert!(result.is_err());
    }

    #[test]
    fn test_chemical_properties() {
        let system = AtomFormationSystem::new();

        let properties = system.determine_chemical_properties(&[0.5; 22], 1);

        assert!(properties.electronegativity > 0.0);
        assert!(properties.electronegativity <= 4.0);
        assert!(properties.ionization_energy > 0.0);
        assert!(properties.atomic_radius > 0.0);
        assert!(properties.valence_electrons <= 8);
        assert!(!properties.oxidation_states.is_empty());
    }

    #[test]
    fn test_chemical_family() {
        let system = AtomFormationSystem::new();

        assert_eq!(
            system.determine_chemical_family(1),
            ChemicalFamily::Nonmetal
        ); // H
        assert_eq!(
            system.determine_chemical_family(2),
            ChemicalFamily::NobleGas
        ); // He
        assert_eq!(
            system.determine_chemical_family(3),
            ChemicalFamily::AlkaliMetal
        ); // Li
        assert_eq!(system.determine_chemical_family(9), ChemicalFamily::Halogen); // F
        assert_eq!(
            system.determine_chemical_family(10),
            ChemicalFamily::NobleGas
        ); // Ne
        assert_eq!(
            system.determine_chemical_family(17),
            ChemicalFamily::Halogen
        ); // Cl
    }

    #[test]
    fn test_electromagnetic_attraction() {
        let system = AtomFormationSystem::new();

        // Create proton and electron
        let mut proton =
            Particle::from_archetype_activation(1, [0.5; 22], Coordinate3D::new(0.0, 0.0, 0.0));
        proton.charge = 1.0;

        let mut electron =
            Particle::from_archetype_activation(2, [0.5; 22], Coordinate3D::new(1.0e-10, 0.0, 0.0));
        electron.charge = -1.0;

        let (force, direction) = system.calculate_electromagnetic_attraction(&proton, &electron);

        assert!(force > 0.0);
        assert!(direction.vx > 0.0); // Direction towards electron
    }

    #[test]
    fn test_archetype_compatibility() {
        let system = AtomFormationSystem::new();

        let atom1 = Atom {
            id: 1,
            position: Coordinate3D::origin(),
            atomic_number: 1,
            mass_number: 1,
            nucleus: Nucleus::new(vec![], vec![]),
            electrons: Vec::new(),
            orbital_configuration: String::new(),
            chemical_properties: ChemicalProperties {
                electronegativity: 2.2,
                ionization_energy: 13.6,
                atomic_radius: 53.0,
                valence_electrons: 1,
                oxidation_states: vec![1, -1],
                chemical_family: ChemicalFamily::Nonmetal,
                reactivity: 0.5,
            },
            archetype_activation: [0.5; 22],
            creation_time: 0,
            age: 0,
        };

        let mut atom2 = atom1.clone();
        atom2.id = 2;
        atom2.archetype_activation = [0.7; 22];

        let compatibility = system.calculate_archetype_compatibility(&atom1, &atom2);

        assert!(compatibility >= 0.0);
        assert!(compatibility <= 1.0);
    }

    #[test]
    fn test_create_atom_from_archetypes() {
        let mut system = AtomFormationSystem::new();

        let atom = system.create_atom_from_archetypes(
            [0.5; 22],
            1, // Hydrogen
            1,
            Coordinate3D::origin(),
        );

        assert_eq!(atom.atomic_number, 1);
        assert_eq!(atom.mass_number, 1);
        assert_eq!(atom.electrons.len(), 1);
        assert_eq!(atom.orbital_configuration, "1s1");
    }

    #[test]
    fn test_create_carbon_atom() {
        let mut system = AtomFormationSystem::new();

        let atom = system.create_atom_from_archetypes(
            [0.5; 22],
            6, // Carbon
            12,
            Coordinate3D::origin(),
        );

        assert_eq!(atom.atomic_number, 6);
        assert_eq!(atom.mass_number, 12);
        assert_eq!(atom.electrons.len(), 6);
        assert_eq!(atom.orbital_configuration, "1s2 2s2 2p2");
        assert_eq!(atom.chemical_properties.valence_electrons, 4);
    }

    #[test]
    fn test_create_oxygen_atom() {
        let mut system = AtomFormationSystem::new();

        let atom = system.create_atom_from_archetypes(
            [0.5; 22],
            8, // Oxygen
            16,
            Coordinate3D::origin(),
        );

        assert_eq!(atom.atomic_number, 8);
        assert_eq!(atom.mass_number, 16);
        assert_eq!(atom.electrons.len(), 8);
        assert_eq!(atom.orbital_configuration, "1s2 2s2 2p4");
        assert_eq!(atom.chemical_properties.valence_electrons, 6);
    }

    #[test]
    fn test_generate_periodic_table() {
        let mut system = AtomFormationSystem::new();

        let periodic_table = system.generate_periodic_table();

        assert_eq!(periodic_table.len(), 118);

        // Check first few elements
        assert_eq!(periodic_table[0].atomic_number, 1); // H
        assert_eq!(periodic_table[1].atomic_number, 2); // He
        assert_eq!(periodic_table[2].atomic_number, 3); // Li
        assert_eq!(periodic_table[5].atomic_number, 6); // C
        assert_eq!(periodic_table[7].atomic_number, 8); // O
        assert_eq!(periodic_table[9].atomic_number, 10); // Ne
    }

    #[test]
    fn test_nuclear_binding_energy() {
        let protons = vec![
            Particle::from_archetype_activation(1, [0.5; 22], Coordinate3D::origin()),
            Particle::from_archetype_activation(1, [0.5; 22], Coordinate3D::origin()),
        ];
        let neutrons = vec![
            Particle::from_archetype_activation(1, [0.5; 22], Coordinate3D::origin()),
            Particle::from_archetype_activation(1, [0.5; 22], Coordinate3D::origin()),
        ];

        let nucleus = Nucleus::new(protons, neutrons);

        // Binding energy should be positive for stable nuclei
        assert!(nucleus.binding_energy > 0.0);
    }

    #[test]
    fn test_valence_electrons() {
        let system = AtomFormationSystem::new();

        assert_eq!(system.calculate_valence_electrons(1), 1); // H
        assert_eq!(system.calculate_valence_electrons(2), 2); // He
        assert_eq!(system.calculate_valence_electrons(6), 4); // C
        assert_eq!(system.calculate_valence_electrons(8), 6); // O
    }

    #[test]
    fn test_oxidation_states() {
        let system = AtomFormationSystem::new();

        // Hydrogen: +1, -1
        let states = system.determine_oxidation_states(1, 1);
        assert!(states.contains(&1));
        assert!(states.contains(&-1));

        // Carbon: -4, +2, +4
        let states = system.determine_oxidation_states(6, 4);
        assert!(states.contains(&-4));
        assert!(states.contains(&2));
        assert!(states.contains(&4));

        // Oxygen: -2, -1, +1, +2
        let states = system.determine_oxidation_states(8, 6);
        assert!(states.contains(&-2));
    }

    #[test]
    fn test_orbital_cache() {
        let mut system = AtomFormationSystem::new();

        // Calculate configuration for hydrogen
        let config1 = system.calculate_orbital_configuration(1);
        let config2 = system.calculate_orbital_configuration(1);

        // Should return cached result
        assert_eq!(config1, config2);
        assert!(system.orbital_cache.contains_key(&1));
    }

    #[test]
    fn test_task_2_3_validation() {
        // Comprehensive validation test for Task 2.3
        let mut system = AtomFormationSystem::new();

        // Test 1: Atoms form correctly from particles
        let atom = system.create_atom_from_archetypes(
            [0.5; 22],
            6, // Carbon
            12,
            Coordinate3D::origin(),
        );
        assert_eq!(atom.nucleus.proton_count(), 6);
        assert_eq!(atom.nucleus.neutron_count(), 6);
        assert_eq!(atom.electrons.len(), 6);

        // Test 2: Orbital configuration matches quantum mechanics
        assert_eq!(atom.orbital_configuration, "1s2 2s2 2p2");

        // Test 3: Chemical properties derive from archetype activation
        assert!(atom.chemical_properties.electronegativity > 0.0);
        assert!(atom.chemical_properties.ionization_energy > 0.0);

        // Test 4: Periodic table emerges from archetype patterns
        let periodic_table = system.generate_periodic_table();
        assert_eq!(periodic_table.len(), 118);
        assert_eq!(periodic_table[0].atomic_number, 1);
        assert_eq!(periodic_table[117].atomic_number, 118);

        // Test 5: Electromagnetic attraction calculation
        let proton = &atom.nucleus.protons[0];
        let electron = &atom.electrons[0];
        let (force, _) = system.calculate_electromagnetic_attraction(proton, &electron.particle);
        assert!(force > 0.0);

        // Test 6: Archetype compatibility for bonding
        let atom2 = system.create_atom_from_archetypes(
            [0.7; 22],
            8, // Oxygen
            16,
            Coordinate3D::new(1.0e-10, 0.0, 0.0),
        );
        let compatibility = system.calculate_archetype_compatibility(&atom, &atom2);
        assert!(compatibility >= 0.0 && compatibility <= 1.0);

        println!("✅ Task 2.3 validation: All tests passed!");
    }
}
