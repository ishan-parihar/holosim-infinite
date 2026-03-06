// Solar System Constraints - Physical laws and planetary biases
//
// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 2.4
// "The Solar/Planetary Logos applies specific Physical Laws and Planetary Biases"
//
// Phase 4 Update: Physical Laws emerge from Logos' archetype choices
// Knowledge Base Reference: REFACTOR_ROADMAP_V2.md Phase 4
// "Physical constants emerge from Logos' choices"

use crate::entity_layer7::holographic_blueprint::HolographicSeed;
use crate::types::Float;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// SolarSystemConstraints defines the local physical laws and planetary biases
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 2.4
/// "The Solar/Planetary Logos applies specific Physical Laws and Planetary Biases"
///
/// Phase 4 Update: Physical Laws emerge from Logos' archetype choices
/// Knowledge Base Reference: REFACTOR_ROADMAP_V2.md Phase 4
/// "Physical constants emerge from Logos' choices"
#[derive(Debug, Clone)]
pub struct SolarSystemConstraints {
    /// Physical constants for this instance
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Applies specific Physical Laws (gravity, electromagnetism, strong/weak forces)"
    ///
    /// Phase 4: These constants emerge from Logos' archetype choices
    pub physics_constants: PhysicsConstants,

    /// Planetary biases (atmosphere, minerals, EM field)
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Applies Planetary Biases (atmospheric composition, mineral content, electromagnetic field)"
    pub planetary_biases: PlanetaryBiases,

    /// Local evolutionary conditions
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Creates the specific electromagnetic field conditions for the simulation instance"
    pub evolutionary_conditions: EvolutionaryConditions,

    /// Phase 4: Logos' archetype choices that determine physical constants
    ///
    /// The Solar Logos chose specific archetype activation patterns that produce
    /// the physical constants observed in this solar system.
    ///
    /// Knowledge Base Reference: REFACTOR_ROADMAP_V2.md Section 2.4
    /// "Logos chose specific patterns that produce known physics"
    pub logos_choices: [Float; 22],

    /// Phase 4: Holographic reference to the seed
    ///
    /// Maintains the fractal-holographic principle: "part contains the whole"
    ///
    /// Knowledge Base Reference: REFACTOR_ROADMAP_V2.md Section 1.3
    /// "Every entity contains the whole"
    ///
    /// Note: This is optional to avoid circular dependency with HolographicSeed
    /// When HolographicSeed is created, it creates SolarSystemConstraints without
    /// a holographic reference, then sets the reference afterwards.
    pub holographic_ref: Option<Arc<HolographicSeed>>,
}

impl SolarSystemConstraints {
    /// Create a new solar system configuration
    ///
    /// Phase 4: Updated to accept Logos' choices and optional holographic reference
    pub fn new(
        physics_constants: PhysicsConstants,
        planetary_biases: PlanetaryBiases,
        evolutionary_conditions: EvolutionaryConditions,
        logos_choices: [Float; 22],
        holographic_ref: Option<Arc<HolographicSeed>>,
    ) -> Self {
        Self {
            physics_constants,
            planetary_biases,
            evolutionary_conditions,
            logos_choices,
            holographic_ref,
        }
    }

    /// Create a new solar system configuration from Logos' archetype choices
    ///
    /// Phase 4: Physical constants emerge from Logos' archetype choices
    ///
    /// Knowledge Base Reference: REFACTOR_ROADMAP_V2.md Section 2.4
    /// "Logos chose specific patterns that produce known physics"
    pub fn from_logos_choices(
        logos_choices: [Float; 22],
        holographic_ref: Option<Arc<HolographicSeed>>,
        planetary_biases: PlanetaryBiases,
        evolutionary_conditions: EvolutionaryConditions,
    ) -> Self {
        // Derive physical constants from Logos' archetype choices
        let physics_constants = PhysicsConstants::derive_from_archetypes(&logos_choices);

        Self {
            physics_constants,
            planetary_biases,
            evolutionary_conditions,
            logos_choices,
            holographic_ref,
        }
    }

    /// Check if solar system constraints are valid
    ///
    /// Valid constraints have:
    /// - Valid physics constants
    /// - Valid planetary biases
    /// - Valid evolutionary conditions
    pub fn is_valid(&self) -> bool {
        // Check physics constants are valid
        if !self.physics_constants.is_valid() {
            return false;
        }

        // Physics constants should be positive
        if self.physics_constants.gravity <= 0.0
            || self.physics_constants.electromagnetism <= 0.0
            || self.physics_constants.strong_force <= 0.0
            || self.physics_constants.weak_force <= 0.0
            || self.physics_constants.speed_of_light <= 0.0
            || self.physics_constants.planck_constant <= 0.0
        {
            return false;
        }

        true
    }

    /// Create a default Earth-like solar system configuration
    ///
    /// Phase 4: Uses Earth-like Logos' archetype choices
    pub fn earth_like() -> Self {
        let logos_choices = PhysicsConstants::earth_like_logos_choices();
        // Note: holographic_ref is None to avoid circular dependency
        // It can be set later using set_holographic_ref()
        let holographic_ref: Option<Arc<HolographicSeed>> = None;

        Self {
            physics_constants: PhysicsConstants::earth_like(),
            planetary_biases: PlanetaryBiases::earth_like(),
            evolutionary_conditions: EvolutionaryConditions::earth_like(),
            logos_choices,
            holographic_ref,
        }
    }

    /// Create a Mars-like solar system configuration
    ///
    /// Phase 4: Uses Mars-like Logos' archetype choices
    pub fn mars_like() -> Self {
        let logos_choices = PhysicsConstants::mars_like_logos_choices();
        // Note: holographic_ref is None to avoid circular dependency
        let holographic_ref: Option<Arc<HolographicSeed>> = None;

        Self {
            physics_constants: PhysicsConstants::mars_like(),
            planetary_biases: PlanetaryBiases::mars_like(),
            evolutionary_conditions: EvolutionaryConditions::mars_like(),
            logos_choices,
            holographic_ref,
        }
    }

    /// Create a custom solar system configuration
    ///
    /// Phase 4: Updated to accept Logos' choices and optional holographic reference
    #[allow(clippy::too_many_arguments)]
    pub fn custom(
        gravity: Float,
        electromagnetism: Float,
        strong_force: Float,
        weak_force: Float,
        speed_of_light: Float,
        planck_constant: Float,
        logos_choices: [Float; 22],
        holographic_ref: Option<Arc<HolographicSeed>>,
    ) -> Self {
        Self {
            physics_constants: PhysicsConstants {
                gravity,
                electromagnetism,
                strong_force,
                weak_force,
                speed_of_light,
                planck_constant,
            },
            planetary_biases: PlanetaryBiases::default(),
            evolutionary_conditions: EvolutionaryConditions::default(),
            logos_choices,
            holographic_ref,
        }
    }

    /// Phase 4: Get Logos' archetype choices
    pub fn logos_choices(&self) -> &[Float; 22] {
        &self.logos_choices
    }

    /// Phase 4: Get holographic reference
    pub fn holographic_ref(&self) -> &Option<Arc<HolographicSeed>> {
        &self.holographic_ref
    }

    /// Phase 4: Set Logos' archetype choices and recalculate constants
    pub fn set_logos_choices(&mut self, logos_choices: [Float; 22]) {
        self.logos_choices = logos_choices;
        // Recalculate physics constants from new archetype choices
        self.physics_constants = PhysicsConstants::derive_from_archetypes(&logos_choices);
    }

    /// Phase 4: Set holographic reference
    pub fn set_holographic_ref(&mut self, holographic_ref: Arc<HolographicSeed>) {
        self.holographic_ref = Some(holographic_ref);
    }

    /// Phase 4: Check if holographic reference is set
    pub fn has_holographic_ref(&self) -> bool {
        self.holographic_ref.is_some()
    }
}

impl Default for SolarSystemConstraints {
    fn default() -> Self {
        Self::earth_like()
    }
}

/// Physics Constants for this simulation instance
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 2.4
/// "Applies specific Physical Laws (gravity, electromagnetism, strong/weak forces)"
///
/// Phase 4 Update: Physical constants emerge from Logos' archetype choices
/// Knowledge Base Reference: REFACTOR_ROADMAP_V2.md Phase 4
/// "Physical constants emerge from Logos' choices"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsConstants {
    /// Gravitational constant (G)
    ///
    /// Phase 4: Emerges from Great Way archetypes (A7, A14, A21)
    pub gravity: Float,

    /// Electromagnetic force constant (k_e)
    ///
    /// Phase 4: Emerges from Catalyst archetypes (A3, A10, A17)
    pub electromagnetism: Float,

    /// Strong nuclear force constant
    ///
    /// Phase 4: Emerges from Transformation archetypes (A6, A13, A20)
    pub strong_force: Float,

    /// Weak nuclear force constant
    ///
    /// Phase 4: Emerges from Experience/Significator archetypes (A4, A11, A18, A5, A12, A19)
    pub weak_force: Float,

    /// Speed of light (c)
    ///
    /// Phase 4: Emerges from Experience/Light speed archetypes (A4, A11, A18)
    pub speed_of_light: Float,

    /// Planck's constant (h)
    ///
    /// Phase 4: Emerges from Quantum/Potentiator archetypes (A2, A9, A16)
    pub planck_constant: Float,
}

impl PhysicsConstants {
    /// Create new physics constants
    pub fn new(
        gravity: Float,
        electromagnetism: Float,
        strong_force: Float,
        weak_force: Float,
        speed_of_light: Float,
        planck_constant: Float,
    ) -> Self {
        Self {
            gravity,
            electromagnetism,
            strong_force,
            weak_force,
            speed_of_light,
            planck_constant,
        }
    }

    /// Create Earth-like physics constants
    pub fn earth_like() -> Self {
        Self {
            gravity: 6.674e-11,          // G in m³/kg·s²
            electromagnetism: 8.987e9,   // k_e in N·m²/C²
            strong_force: 1.0,           // Normalized strong force
            weak_force: 1.0,             // Normalized weak force
            speed_of_light: 299792458.0, // c in m/s
            planck_constant: 6.626e-34,  // h in J·s
        }
    }

    /// Create Mars-like physics constants (slightly different gravity)
    pub fn mars_like() -> Self {
        Self {
            gravity: 6.674e-11,        // Same G
            electromagnetism: 8.987e9, // Same k_e
            strong_force: 1.0,
            weak_force: 1.0,
            speed_of_light: 299792458.0,
            planck_constant: 6.626e-34,
        }
    }

    /// Calculate gravitational force magnitude
    ///
    /// F = G * m1 * m2 / r²
    pub fn gravitational_force(&self, m1: Float, m2: Float, distance: Float) -> Float {
        if distance <= 0.0 {
            return 0.0;
        }
        (self.gravity * m1 * m2) / (distance * distance)
    }

    /// Calculate electromagnetic force magnitude
    ///
    /// F = k_e * q1 * q2 / r²
    pub fn electromagnetic_force(&self, q1: Float, q2: Float, distance: Float) -> Float {
        if distance <= 0.0 {
            return 0.0;
        }
        (self.electromagnetism * q1 * q2) / (distance * distance)
    }

    /// Get the ratio of strong to weak force
    pub fn strong_weak_ratio(&self) -> Float {
        self.strong_force / self.weak_force
    }

    /// Check if physics constants are valid
    ///
    /// Valid physics constants have all values within reasonable bounds
    pub fn is_valid(&self) -> bool {
        // All constants should be positive
        if self.gravity <= 0.0
            || self.electromagnetism <= 0.0
            || self.strong_force <= 0.0
            || self.weak_force <= 0.0
            || self.speed_of_light <= 0.0
            || self.planck_constant <= 0.0
        {
            return false;
        }

        // Speed of light should be within reasonable bounds
        if self.speed_of_light < 1.0e6 || self.speed_of_light > 1.0e9 {
            return false;
        }

        // Planck constant should be within reasonable bounds
        if self.planck_constant < 1.0e-40 || self.planck_constant > 1.0e-30 {
            return false;
        }

        true
    }
}

impl Default for PhysicsConstants {
    fn default() -> Self {
        Self::earth_like()
    }
}

impl PhysicsConstants {
    /// Phase 4: Derive physical constants from Logos' archetype choices
    ///
    /// Knowledge Base Reference: REFACTOR_ROADMAP_V2.md Section 2.4
    /// "Constants emerge from Logos' choices"
    ///
    /// # Archetype-to-Constant Mappings
    ///
    /// **Gravitational Constant (G):**
    /// - Emerges from Great Way archetypes (A7, A14, A21)
    /// - Formula: G = G_earth × (gw_activation / 3.0)^2
    /// - Where gw_activation = (A7 + A14 + A21) / 3.0
    ///
    /// **Speed of Light (c):**
    /// - Emerges from Experience/Light speed archetypes (A4, A11, A18)
    /// - Formula: c = c_earth × (exp_activation / 3.0)
    /// - Where exp_activation = (A4 + A11 + A18) / 3.0
    ///
    /// **Planck Constant (h):**
    /// - Emerges from Quantum/Potentiator archetypes (A2, A9, A16)
    /// - Formula: h = h_earth × (pot_activation / 3.0)
    /// - Where pot_activation = (A2 + A9 + A16) / 3.0
    ///
    /// **Electromagnetic Constant (k_e):**
    /// - Emerges from Catalyst archetypes (A3, A10, A17)
    /// - Formula: k_e = k_e_earth × (cat_activation / 3.0)^2
    /// - Where cat_activation = (A3 + A10 + A17) / 3.0
    ///
    /// **Strong Nuclear Force:**
    /// - Emerges from Transformation archetypes (A6, A13, A20)
    /// - Formula: F_strong = F_strong_earth × (trans_activation / 3.0)
    /// - Where trans_activation = (A6 + A13 + A20) / 3.0
    ///
    /// **Weak Nuclear Force:**
    /// - Emerges from Experience/Significator archetypes (A4, A11, A18, A5, A12, A19)
    /// - Formula: F_weak = F_weak_earth × (exp_sig_activation / 6.0)
    /// - Where exp_sig_activation = (A4 + A11 + A18 + A5 + A12 + A19) / 6.0
    ///
    /// # Arguments
    /// * `logos_choices` - Logos' archetype choices (0.0 to 1.0 for each archetype)
    ///
    /// # Returns
    /// Derived physical constants
    pub fn derive_from_archetypes(logos_choices: &[Float; 22]) -> Self {
        // Earth-like constants as baseline
        let g_earth = 6.674e-11; // G in m³/kg·s²
        let k_e_earth = 8.987e9; // k_e in N·m²/C²
        let c_earth = 299792458.0; // c in m/s
        let h_earth = 6.626e-34; // h in J·s
        let f_strong_earth = 1.0; // Normalized strong force
        let f_weak_earth = 1.0; // Normalized weak force

        // Great Way archetypes (A7, A14, A21) - Gravitational constant
        let gw_activation = (logos_choices[6] + logos_choices[13] + logos_choices[20]) / 3.0;
        let gravity = g_earth * (gw_activation.powi(2));

        // Catalyst archetypes (A3, A10, A17) - Electromagnetic constant
        let cat_activation = (logos_choices[2] + logos_choices[9] + logos_choices[16]) / 3.0;
        let electromagnetism = k_e_earth * (cat_activation.powi(2));

        // Experience archetypes (A4, A11, A18) - Speed of light
        let exp_activation = (logos_choices[3] + logos_choices[10] + logos_choices[17]) / 3.0;
        let speed_of_light = c_earth * exp_activation;

        // Potentiator archetypes (A2, A9, A16) - Planck constant
        let pot_activation = (logos_choices[1] + logos_choices[8] + logos_choices[15]) / 3.0;
        let planck_constant = h_earth * pot_activation;

        // Transformation archetypes (A6, A13, A20) - Strong nuclear force
        let trans_activation = (logos_choices[5] + logos_choices[12] + logos_choices[19]) / 3.0;
        let strong_force = f_strong_earth * trans_activation;

        // Experience + Significator archetypes (A4, A11, A18, A5, A12, A19) - Weak nuclear force
        let exp_sig_activation = (logos_choices[3]
            + logos_choices[10]
            + logos_choices[17]
            + logos_choices[4]
            + logos_choices[11]
            + logos_choices[18])
            / 6.0;
        let weak_force = f_weak_earth * exp_sig_activation;

        Self {
            gravity,
            electromagnetism,
            strong_force,
            weak_force,
            speed_of_light,
            planck_constant,
        }
    }

    /// Phase 4: Get Earth-like Logos' archetype choices
    ///
    /// These choices produce Earth-like physical constants
    pub fn earth_like_logos_choices() -> [Float; 22] {
        [
            // Mind Complex (A1-A7)
            1.0, // A1: Matrix
            1.0, // A2: Potentiator
            1.0, // A3: Catalyst
            1.0, // A4: Experience
            1.0, // A5: Significator
            1.0, // A6: Transformation
            1.0, // A7: Great Way
            // Body Complex (A8-A14)
            1.0, // A8: Matrix
            1.0, // A9: Potentiator
            1.0, // A10: Catalyst
            1.0, // A11: Experience
            1.0, // A12: Significator
            1.0, // A13: Transformation
            1.0, // A14: Great Way
            // Spirit Complex (A15-A21)
            1.0, // A15: Matrix
            1.0, // A16: Potentiator
            1.0, // A17: Catalyst
            1.0, // A18: Experience
            1.0, // A19: Significator
            1.0, // A20: Transformation
            1.0, // A21: Great Way
            // Choice (A22)
            1.0, // A22: Choice
        ]
    }

    /// Phase 4: Get Mars-like Logos' archetype choices
    ///
    /// These choices produce Mars-like physical constants
    /// (slightly different gravity and electromagnetic field)
    pub fn mars_like_logos_choices() -> [Float; 22] {
        [
            // Mind Complex (A1-A7)
            1.0,  // A1: Matrix
            1.0,  // A2: Potentiator
            0.95, // A3: Catalyst (slightly weaker EM)
            1.0,  // A4: Experience
            1.0,  // A5: Significator
            1.0,  // A6: Transformation
            1.0,  // A7: Great Way
            // Body Complex (A8-A14)
            1.0,  // A8: Matrix
            1.0,  // A9: Potentiator
            0.95, // A10: Catalyst (slightly weaker EM)
            1.0,  // A11: Experience
            1.0,  // A12: Significator
            1.0,  // A13: Transformation
            1.0,  // A14: Great Way
            // Spirit Complex (A15-A21)
            1.0,  // A15: Matrix
            1.0,  // A16: Potentiator
            0.95, // A17: Catalyst (slightly weaker EM)
            1.0,  // A18: Experience
            1.0,  // A19: Significator
            1.0,  // A20: Transformation
            1.0,  // A21: Great Way
            // Choice (A22)
            1.0, // A22: Choice
        ]
    }

    /// Phase 4: Validate that constants match known values within tolerance
    ///
    /// # Arguments
    /// * `expected` - Expected values for G, c, h
    /// * `tolerance` - Maximum allowed relative error (e.g., 0.001 for 0.1%)
    ///
    /// # Returns
    /// True if all constants are within tolerance
    pub fn validate_constants(&self, expected: (Float, Float, Float), tolerance: Float) -> bool {
        let (expected_g, expected_c, expected_h) = expected;

        // Check gravitational constant
        let g_error = (self.gravity - expected_g).abs() / expected_g;
        if g_error > tolerance {
            return false;
        }

        // Check speed of light
        let c_error = (self.speed_of_light - expected_c).abs() / expected_c;
        if c_error > tolerance {
            return false;
        }

        // Check Planck constant
        let h_error = (self.planck_constant - expected_h).abs() / expected_h;
        if h_error > tolerance {
            return false;
        }

        true
    }
}

/// Planetary Biases - Planet-specific modifiers
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 2.4
/// "Applies Planetary Biases (atmospheric composition, mineral content, electromagnetic field)"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetaryBiases {
    /// Atmospheric composition
    pub atmosphere: AtmosphericComposition,

    /// Mineral content
    pub minerals: MineralComposition,

    /// Electromagnetic field strength
    pub em_field: EMField,

    /// Temperature range (min, max) in Kelvin
    pub temperature_range: (Float, Float),

    /// Radiation levels
    pub radiation_levels: RadiationLevels,
}

impl PlanetaryBiases {
    /// Create new planetary biases
    pub fn new(
        atmosphere: AtmosphericComposition,
        minerals: MineralComposition,
        em_field: EMField,
        temperature_range: (Float, Float),
        radiation_levels: RadiationLevels,
    ) -> Self {
        Self {
            atmosphere,
            minerals,
            em_field,
            temperature_range,
            radiation_levels,
        }
    }

    /// Create Earth-like planetary biases
    pub fn earth_like() -> Self {
        Self {
            atmosphere: AtmosphericComposition::earth_like(),
            minerals: MineralComposition::earth_like(),
            em_field: EMField::earth_like(),
            temperature_range: (288.0, 298.0), // 15-25°C average
            radiation_levels: RadiationLevels::earth_like(),
        }
    }

    /// Create Mars-like planetary biases
    pub fn mars_like() -> Self {
        Self {
            atmosphere: AtmosphericComposition::mars_like(),
            minerals: MineralComposition::mars_like(),
            em_field: EMField::mars_like(),
            temperature_range: (210.0, 248.0), // -63 to -25°C average
            radiation_levels: RadiationLevels::mars_like(),
        }
    }

    /// Check if temperature is within habitable range
    pub fn is_habitable_temperature(&self, temperature: Float) -> bool {
        temperature >= self.temperature_range.0 && temperature <= self.temperature_range.1
    }

    /// Get average temperature
    pub fn average_temperature(&self) -> Float {
        (self.temperature_range.0 + self.temperature_range.1) / 2.0
    }
}

impl Default for PlanetaryBiases {
    fn default() -> Self {
        Self::earth_like()
    }
}

/// Atmospheric Composition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtmosphericComposition {
    /// Nitrogen percentage (0.0-1.0)
    pub nitrogen: Float,

    /// Oxygen percentage (0.0-1.0)
    pub oxygen: Float,

    /// Carbon dioxide percentage (0.0-1.0)
    pub carbon_dioxide: Float,

    /// Other gases percentage (0.0-1.0)
    pub other: Float,

    /// Atmospheric pressure (in Pascals)
    pub pressure: Float,
}

impl AtmosphericComposition {
    /// Create new atmospheric composition
    pub fn new(
        nitrogen: Float,
        oxygen: Float,
        carbon_dioxide: Float,
        other: Float,
        pressure: Float,
    ) -> Self {
        Self {
            nitrogen,
            oxygen,
            carbon_dioxide,
            other,
            pressure,
        }
    }

    /// Create Earth-like atmosphere
    pub fn earth_like() -> Self {
        Self {
            nitrogen: 0.78,
            oxygen: 0.21,
            carbon_dioxide: 0.0004,
            other: 0.0096,
            pressure: 101325.0, // 1 atm in Pa
        }
    }

    /// Create Mars-like atmosphere
    pub fn mars_like() -> Self {
        Self {
            nitrogen: 0.027,
            oxygen: 0.0013,
            carbon_dioxide: 0.953,
            other: 0.0187,
            pressure: 600.0, // ~0.006 atm
        }
    }

    /// Check if atmosphere supports carbon-based life
    pub fn supports_carbon_life(&self) -> bool {
        self.oxygen > 0.1 && self.carbon_dioxide > 0.0001 && self.pressure > 10000.0
    }

    /// Validate that percentages sum to approximately 1.0
    pub fn validate(&self) -> bool {
        let total = self.nitrogen + self.oxygen + self.carbon_dioxide + self.other;
        (total - 1.0).abs() < 0.01
    }
}

impl Default for AtmosphericComposition {
    fn default() -> Self {
        Self::earth_like()
    }
}

/// Mineral Composition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MineralComposition {
    /// Silicon content (0.0-1.0)
    pub silicon: Float,

    /// Iron content (0.0-1.0)
    pub iron: Float,

    /// Aluminum content (0.0-1.0)
    pub aluminum: Float,

    /// Calcium content (0.0-1.0)
    pub calcium: Float,

    /// Other minerals (0.0-1.0)
    pub other: Float,
}

impl MineralComposition {
    /// Create new mineral composition
    pub fn new(silicon: Float, iron: Float, aluminum: Float, calcium: Float, other: Float) -> Self {
        Self {
            silicon,
            iron,
            aluminum,
            calcium,
            other,
        }
    }

    /// Create Earth-like mineral composition
    pub fn earth_like() -> Self {
        Self {
            silicon: 0.28,
            iron: 0.05,
            aluminum: 0.08,
            calcium: 0.04,
            other: 0.55,
        }
    }

    /// Create Mars-like mineral composition
    pub fn mars_like() -> Self {
        Self {
            silicon: 0.20,
            iron: 0.18,
            aluminum: 0.07,
            calcium: 0.03,
            other: 0.52,
        }
    }

    /// Check if composition supports technological development
    pub fn supports_technology(&self) -> bool {
        self.silicon > 0.1 && self.iron > 0.02 && self.aluminum > 0.05
    }
}

impl Default for MineralComposition {
    fn default() -> Self {
        Self::earth_like()
    }
}

/// Electromagnetic Field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EMField {
    /// Field strength (in Tesla)
    pub strength: Float,

    /// Field inclination (in degrees)
    pub inclination: Float,

    /// Field declination (in degrees)
    pub declination: Float,

    /// Field stability (0.0-1.0, 1.0 = perfectly stable)
    pub stability: Float,
}

impl EMField {
    /// Create new EM field
    pub fn new(strength: Float, inclination: Float, declination: Float, stability: Float) -> Self {
        Self {
            strength,
            inclination,
            declination,
            stability,
        }
    }

    /// Create Earth-like EM field
    pub fn earth_like() -> Self {
        Self {
            strength: 0.00005, // 50 μT
            inclination: 60.0,
            declination: 0.0,
            stability: 0.9,
        }
    }

    /// Create Mars-like EM field (very weak)
    pub fn mars_like() -> Self {
        Self {
            strength: 0.0000015, // ~1.5 nT
            inclination: -60.0,
            declination: 0.0,
            stability: 0.5,
        }
    }

    /// Check if EM field provides adequate radiation protection
    pub fn provides_radiation_protection(&self) -> bool {
        self.strength > 0.00001 && self.stability > 0.5
    }

    /// Calculate field intensity at a given latitude
    pub fn intensity_at_latitude(&self, latitude: Float) -> Float {
        let lat_rad = latitude.to_radians();
        self.strength * (1.0 + 0.3 * lat_rad.sin().powi(2))
    }
}

impl Default for EMField {
    fn default() -> Self {
        Self::earth_like()
    }
}

/// Radiation Levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiationLevels {
    /// Cosmic ray intensity (0.0-1.0)
    pub cosmic_rays: Float,

    /// Solar radiation intensity (0.0-1.0)
    pub solar: Float,

    /// Background radiation (0.0-1.0)
    pub background: Float,
}

impl RadiationLevels {
    /// Create new radiation levels
    pub fn new(cosmic_rays: Float, solar: Float, background: Float) -> Self {
        Self {
            cosmic_rays,
            solar,
            background,
        }
    }

    /// Create Earth-like radiation levels
    pub fn earth_like() -> Self {
        Self {
            cosmic_rays: 0.3,
            solar: 0.5,
            background: 0.2,
        }
    }

    /// Create Mars-like radiation levels (higher due to thin atmosphere)
    pub fn mars_like() -> Self {
        Self {
            cosmic_rays: 0.8,
            solar: 0.9,
            background: 0.4,
        }
    }

    /// Calculate total radiation load
    pub fn total_radiation(&self) -> Float {
        self.cosmic_rays + self.solar + self.background
    }

    /// Check if radiation levels are survivable
    pub fn is_survivable(&self) -> bool {
        self.total_radiation() < 2.0
    }
}

impl Default for RadiationLevels {
    fn default() -> Self {
        Self::earth_like()
    }
}

/// Evolutionary Conditions - Local conditions affecting evolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionaryConditions {
    /// Evolutionary speed (0.0-1.0, 1.0 = fastest)
    pub evolutionary_speed: Float,

    /// Mutation rate (0.0-1.0)
    pub mutation_rate: Float,

    /// Complexity capacity (0.0-1.0)
    pub complexity_capacity: Float,

    /// Consciousness emergence threshold (0.0-1.0)
    pub consciousness_threshold: Float,
}

impl EvolutionaryConditions {
    /// Create new evolutionary conditions
    pub fn new(
        evolutionary_speed: Float,
        mutation_rate: Float,
        complexity_capacity: Float,
        consciousness_threshold: Float,
    ) -> Self {
        Self {
            evolutionary_speed,
            mutation_rate,
            complexity_capacity,
            consciousness_threshold,
        }
    }

    /// Create Earth-like evolutionary conditions
    pub fn earth_like() -> Self {
        Self {
            evolutionary_speed: 0.5,
            mutation_rate: 0.5,
            complexity_capacity: 0.8,
            consciousness_threshold: 0.5,
        }
    }

    /// Create Mars-like evolutionary conditions
    pub fn mars_like() -> Self {
        Self {
            evolutionary_speed: 0.3,
            mutation_rate: 0.7, // Higher due to radiation
            complexity_capacity: 0.6,
            consciousness_threshold: 0.6,
        }
    }

    /// Check if conditions support consciousness emergence
    pub fn supports_consciousness(&self, awareness_level: Float) -> bool {
        awareness_level >= self.consciousness_threshold
    }

    /// Calculate evolutionary pressure (combination of speed and mutation)
    pub fn evolutionary_pressure(&self) -> Float {
        (self.evolutionary_speed + self.mutation_rate) / 2.0
    }
}

impl Default for EvolutionaryConditions {
    fn default() -> Self {
        Self::earth_like()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physics_constants_creation() {
        let constants = PhysicsConstants::earth_like();
        assert!(constants.gravity > 0.0);
        assert!(constants.electromagnetism > 0.0);
        assert!(constants.speed_of_light > 0.0);
    }

    #[test]
    fn test_gravitational_force_calculation() {
        let constants = PhysicsConstants::earth_like();
        let force = constants.gravitational_force(100.0, 200.0, 10.0);
        assert!(force > 0.0);
    }

    #[test]
    fn test_electromagnetic_force_calculation() {
        let constants = PhysicsConstants::earth_like();
        let force = constants.electromagnetic_force(1.0, -1.0, 10.0);
        assert!(force < 0.0); // Attractive force
    }

    #[test]
    fn test_atmospheric_composition_validation() {
        let atmosphere = AtmosphericComposition::earth_like();
        assert!(atmosphere.validate());
    }

    #[test]
    fn test_em_field_protection() {
        let earth_field = EMField::earth_like();
        assert!(earth_field.provides_radiation_protection());

        let mars_field = EMField::mars_like();
        assert!(!mars_field.provides_radiation_protection());
    }

    #[test]
    fn test_radiation_levels() {
        let earth_radiation = RadiationLevels::earth_like();
        assert!(earth_radiation.is_survivable());
        assert!(earth_radiation.total_radiation() < 2.0);
    }

    #[test]
    fn test_solar_system_constraints() {
        let constraints = SolarSystemConstraints::earth_like();
        assert!(constraints.physics_constants.gravity > 0.0);
        assert!(constraints.planetary_biases.atmosphere.validate());
    }

    #[test]
    fn test_evolutionary_conditions() {
        let conditions = EvolutionaryConditions::earth_like();
        assert!(conditions.evolutionary_speed > 0.0);
        assert!(conditions.evolutionary_pressure() > 0.0);
    }

    // Phase 4 Tests: Physical Laws Emergence

    #[test]
    fn test_logos_choices_in_solar_system_constraints() {
        let constraints = SolarSystemConstraints::earth_like();
        let logos_choices = constraints.logos_choices();

        // Logos' choices should be present
        assert_eq!(logos_choices.len(), 22);

        // All choices should be positive
        for choice in logos_choices.iter() {
            assert!(*choice > 0.0);
        }
    }

    #[test]
    fn test_holographic_ref_in_solar_system_constraints() {
        let constraints = SolarSystemConstraints::earth_like();
        let holographic_ref = constraints.holographic_ref();

        // Holographic reference should be None by default (to avoid circular dependency)
        assert!(holographic_ref.is_none());

        // Can be set later
        let new_ref = Arc::new(HolographicSeed::default());
        let mut constraints = SolarSystemConstraints::earth_like();
        constraints.set_holographic_ref(new_ref);

        // Now holographic reference should be present
        assert!(constraints.has_holographic_ref());
        let ref_opt = constraints.holographic_ref();
        assert!(ref_opt.is_some());
        assert!(ref_opt.as_ref().unwrap().free_will.free_will_intensity > 0.0);
    }

    #[test]
    fn test_derive_constants_from_archetypes() {
        let logos_choices = PhysicsConstants::earth_like_logos_choices();
        let constants = PhysicsConstants::derive_from_archetypes(&logos_choices);

        // Constants should be positive
        assert!(constants.gravity > 0.0);
        assert!(constants.electromagnetism > 0.0);
        assert!(constants.strong_force > 0.0);
        assert!(constants.weak_force > 0.0);
        assert!(constants.speed_of_light > 0.0);
        assert!(constants.planck_constant > 0.0);
    }

    #[test]
    fn test_gravitational_constant_emergence_from_great_way() {
        let logos_choices = PhysicsConstants::earth_like_logos_choices();

        // Great Way archetypes (A7, A14, A21) control gravitational constant
        let gw_activation = (logos_choices[6] + logos_choices[13] + logos_choices[20]) / 3.0;
        let constants = PhysicsConstants::derive_from_archetypes(&logos_choices);

        // G should be proportional to (gw_activation)^2
        let g_earth = 6.674e-11;
        let expected_g = g_earth * (gw_activation.powi(2));

        assert!((constants.gravity - expected_g).abs() < 1e-15);
    }

    #[test]
    fn test_speed_of_light_emergence_from_experience() {
        let logos_choices = PhysicsConstants::earth_like_logos_choices();

        // Experience archetypes (A4, A11, A18) control speed of light
        let exp_activation = (logos_choices[3] + logos_choices[10] + logos_choices[17]) / 3.0;
        let constants = PhysicsConstants::derive_from_archetypes(&logos_choices);

        // c should be proportional to exp_activation
        let c_earth = 299792458.0;
        let expected_c = c_earth * exp_activation;

        assert!((constants.speed_of_light - expected_c).abs() < 1e-6);
    }

    #[test]
    fn test_planck_constant_emergence_from_potentiator() {
        let logos_choices = PhysicsConstants::earth_like_logos_choices();

        // Potentiator archetypes (A2, A9, A16) control Planck constant
        let pot_activation = (logos_choices[1] + logos_choices[8] + logos_choices[15]) / 3.0;
        let constants = PhysicsConstants::derive_from_archetypes(&logos_choices);

        // h should be proportional to pot_activation
        let h_earth = 6.626e-34;
        let expected_h = h_earth * pot_activation;

        assert!((constants.planck_constant - expected_h).abs() < 1e-38);
    }

    #[test]
    fn test_electromagnetic_constant_emergence_from_catalyst() {
        let logos_choices = PhysicsConstants::earth_like_logos_choices();

        // Catalyst archetypes (A3, A10, A17) control electromagnetic constant
        let cat_activation = (logos_choices[2] + logos_choices[9] + logos_choices[16]) / 3.0;
        let constants = PhysicsConstants::derive_from_archetypes(&logos_choices);

        // k_e should be proportional to (cat_activation)^2
        let k_e_earth = 8.987e9;
        let expected_k_e = k_e_earth * (cat_activation.powi(2));

        assert!((constants.electromagnetism - expected_k_e).abs() < 1e5);
    }

    #[test]
    fn test_strong_force_emergence_from_transformation() {
        let logos_choices = PhysicsConstants::earth_like_logos_choices();

        // Transformation archetypes (A6, A13, A20) control strong force
        let trans_activation = (logos_choices[5] + logos_choices[12] + logos_choices[19]) / 3.0;
        let constants = PhysicsConstants::derive_from_archetypes(&logos_choices);

        // Strong force should be proportional to trans_activation
        let f_strong_earth = 1.0;
        let expected_strong = f_strong_earth * trans_activation;

        assert!((constants.strong_force - expected_strong).abs() < 1e-10);
    }

    #[test]
    fn test_weak_force_emergence_from_experience_significator() {
        let logos_choices = PhysicsConstants::earth_like_logos_choices();

        // Experience + Significator archetypes control weak force
        let exp_sig_activation = (logos_choices[3]
            + logos_choices[10]
            + logos_choices[17]
            + logos_choices[4]
            + logos_choices[11]
            + logos_choices[18])
            / 6.0;
        let constants = PhysicsConstants::derive_from_archetypes(&logos_choices);

        // Weak force should be proportional to exp_sig_activation
        let f_weak_earth = 1.0;
        let expected_weak = f_weak_earth * exp_sig_activation;

        assert!((constants.weak_force - expected_weak).abs() < 1e-10);
    }

    #[test]
    fn test_validate_constants_within_tolerance() {
        let logos_choices = PhysicsConstants::earth_like_logos_choices();
        let constants = PhysicsConstants::derive_from_archetypes(&logos_choices);

        // Expected Earth values
        let expected = (6.674e-11, 299792458.0, 6.626e-34);

        // Should validate within 0.1% tolerance
        assert!(constants.validate_constants(expected, 0.001));

        // Should validate within 0.0001% tolerance (Earth-like choices produce exact values)
        assert!(constants.validate_constants(expected, 0.000001));
    }

    #[test]
    fn test_validate_constants_with_different_choices() {
        // Use Mars-like choices (slightly different from Earth)
        let logos_choices = PhysicsConstants::mars_like_logos_choices();
        let constants = PhysicsConstants::derive_from_archetypes(&logos_choices);

        // Expected Earth values (Mars choices should produce slightly different values)
        let expected = (6.674e-11, 299792458.0, 6.626e-34);

        // Mars-like choices only differ in Catalyst archetypes (0.95 vs 1.0)
        // This affects electromagnetic constant, but G, c, h remain the same
        // So they should still validate within 0.0001% tolerance
        assert!(constants.validate_constants(expected, 0.000001));
    }

    #[test]
    fn test_validate_constants_with_custom_choices() {
        // Use custom choices with different Great Way activation
        let mut custom_choices = PhysicsConstants::earth_like_logos_choices();
        custom_choices[6] = 0.8; // Reduce A7 activation
        custom_choices[13] = 0.8; // Reduce A14 activation
        custom_choices[20] = 0.8; // Reduce A21 activation

        let constants = PhysicsConstants::derive_from_archetypes(&custom_choices);

        // Expected Earth values
        let expected = (6.674e-11, 299792458.0, 6.626e-34);

        // Should validate within 50% tolerance (significant difference in G)
        assert!(constants.validate_constants(expected, 0.5));

        // Should not validate within 0.1% tolerance (G is significantly different)
        assert!(!constants.validate_constants(expected, 0.001));
    }

    #[test]
    fn test_different_solar_systems_different_constants() {
        let earth_choices = PhysicsConstants::earth_like_logos_choices();
        let mars_choices = PhysicsConstants::mars_like_logos_choices();

        let earth_constants = PhysicsConstants::derive_from_archetypes(&earth_choices);
        let mars_constants = PhysicsConstants::derive_from_archetypes(&mars_choices);

        // Mars should have slightly different electromagnetic constant
        // due to weaker Catalyst activation (0.95 vs 1.0)
        assert!((earth_constants.electromagnetism - mars_constants.electromagnetism).abs() > 1e7);
    }

    #[test]
    fn test_solar_system_constraints_from_logos_choices() {
        let logos_choices = PhysicsConstants::earth_like_logos_choices();
        // holographic_ref is None to avoid circular dependency
        let holographic_ref: Option<Arc<HolographicSeed>> = None;

        let constraints = SolarSystemConstraints::from_logos_choices(
            logos_choices,
            holographic_ref,
            PlanetaryBiases::earth_like(),
            EvolutionaryConditions::earth_like(),
        );

        // Constants should be derived from archetype choices
        assert!(constraints.physics_constants.gravity > 0.0);
        assert!(constraints.physics_constants.speed_of_light > 0.0);
        assert!(constraints.physics_constants.planck_constant > 0.0);

        // Logos' choices should be stored
        assert_eq!(constraints.logos_choices, logos_choices);
    }

    #[test]
    fn test_set_logos_choices_recalculates_constants() {
        let mut constraints = SolarSystemConstraints::earth_like();

        // Get original constants
        let original_gravity = constraints.physics_constants.gravity;

        // Create new Logos' choices with different Great Way activation
        let mut new_choices = *constraints.logos_choices();
        new_choices[6] = 0.8; // Reduce A7 activation
        new_choices[13] = 0.8; // Reduce A14 activation
        new_choices[20] = 0.8; // Reduce A21 activation

        // Set new choices
        constraints.set_logos_choices(new_choices);

        // Gravitational constant should change
        let new_gravity = constraints.physics_constants.gravity;
        assert!((original_gravity - new_gravity).abs() > 1e-12);
    }

    #[test]
    fn test_custom_solar_system_with_logos_choices() {
        let logos_choices = [0.5; 22];
        // holographic_ref is None to avoid circular dependency
        let holographic_ref: Option<Arc<HolographicSeed>> = None;

        let constraints = SolarSystemConstraints::custom(
            6.674e-11,
            8.987e9,
            1.0,
            1.0,
            299792458.0,
            6.626e-34,
            logos_choices,
            holographic_ref,
        );

        // Logos' choices should be stored
        assert_eq!(constraints.logos_choices, [0.5; 22]);
    }
}
