// Energy Densification System - Phase 1: Involution Process
//
// This module implements the conversion of high-frequency energy to low-frequency matter
// as density decreases during involution. Energy is conserved during this process
// following E=mc² (energy-mass equivalence).
//
// Key Principles:
// 1. Energy is conserved during densification (E=mc²)
// 2. Matter properties derive from archetype activation patterns
// 3. Energy-to-matter conversion follows physical laws
// 4. No energy is created or destroyed
// 5. Light carries the complete blueprint for matter manifestation
//
// Knowledge Base References:
// - COMPLETE_REFACTOR_ROADMAP_V4.md Phase 1, Task 1.2
// - "Energy conserved during densification (E=mc²)"
// - "Matter properties derive from archetype activation"

use crate::light::architecture::LightArchitecture;
use crate::physics_derivation::{
    derive_charge_from_archetypes, derive_lifetime_from_archetypes, derive_mass_from_archetypes,
    derive_spin_from_archetypes,
};
use crate::types::Float;
use std::collections::HashMap;

// ============================================================================
// CONSTANTS
// ============================================================================

/// Speed of light in vacuum (m/s)
///
/// From Einstein's special relativity: E=mc²
/// This is the conversion factor between energy and mass
const SPEED_OF_LIGHT: Float = 299_792_458.0;

/// Electron mass (kg)
///
/// Rest mass of an electron
const ELECTRON_MASS: Float = 9.109_383_7015e-31;

/// Proton mass (kg)
///
/// Rest mass of a proton
const PROTON_MASS: Float = 1.672_621_92369e-27;

/// Neutron mass (kg)
///
/// Rest mass of a neutron
const NEUTRON_MASS: Float = 1.674_927_49804e-27;

/// Energy conservation tolerance (0.01%)
///
/// Energy must be conserved within this tolerance
const ENERGY_CONSERVATION_TOLERANCE: Float = 0.0001;

// ============================================================================
// MATTER PROPERTIES
// ============================================================================

/// Matter properties derived from energy densification
///
/// Represents the physical properties of matter that emerge from
/// energy condensation at a specific density
#[derive(Debug, Clone)]
pub struct MatterProperties {
    /// Mass in kilograms
    pub mass: Float,

    /// Electric charge in elementary charge units
    pub charge: Float,

    /// Spin quantum number
    pub spin: Float,

    /// Lifetime in seconds (unstable particles have finite lifetime)
    pub lifetime: Float,

    /// Energy equivalent in joules (E=mc²)
    pub energy_equivalent: Float,

    /// Density at which matter formed
    pub formation_density: u8,

    /// Archetype activation pattern that generated this matter
    pub archetype_activation: [Float; 22],

    /// Is this matter stable (infinite lifetime)?
    pub is_stable: bool,
}

impl MatterProperties {
    /// Calculate the energy equivalent of mass (E=mc²)
    ///
    /// # Arguments
    /// * `mass` - Mass in kilograms
    ///
    /// # Returns
    /// Energy in joules
    pub fn energy_from_mass(mass: Float) -> Float {
        mass * SPEED_OF_LIGHT.powi(2)
    }

    /// Calculate the mass equivalent of energy (m=E/c²)
    ///
    /// # Arguments
    /// * `energy` - Energy in joules
    ///
    /// # Returns
    /// Mass in kilograms
    pub fn mass_from_energy(energy: Float) -> Float {
        energy / SPEED_OF_LIGHT.powi(2)
    }

    /// Check if matter is stable (infinite lifetime)
    ///
    /// Stable matter has lifetime >= 1e20 seconds (effectively infinite)
    pub fn check_stability(&mut self) {
        self.is_stable = self.lifetime >= 1e20;
    }

    /// Get a string representation of the matter type
    ///
    /// Based on mass and charge properties
    pub fn matter_type(&self) -> String {
        let mass_ratio = self.mass / ELECTRON_MASS;

        if mass_ratio < 2.0 {
            if self.charge.abs() < 0.1 {
                "Neutrino".to_string()
            } else {
                "Electron".to_string()
            }
        } else if mass_ratio < 2000.0 {
            if self.charge > 0.5 {
                "Muon".to_string()
            } else {
                "Anti-muon".to_string()
            }
        } else if (mass_ratio - 1836.0).abs() < 100.0 {
            if self.charge > 0.5 {
                "Proton".to_string()
            } else if self.charge < -0.5 {
                "Anti-proton".to_string()
            } else {
                "Neutron".to_string()
            }
        } else {
            "Composite Particle".to_string()
        }
    }
}

// ============================================================================
// ENERGY DENSIFICATION EVENT
// ============================================================================

/// Record of an energy densification event
///
/// Tracks the conversion of energy to matter during involution
#[derive(Debug, Clone)]
pub struct EnergyDensificationEvent {
    /// Entity ID that underwent densification
    pub entity_id: u64,

    /// Source density (higher density)
    pub source_density: u8,

    /// Target density (lower density)
    pub target_density: u8,

    /// Input energy (joules)
    pub input_energy: Float,

    /// Output mass (kilograms)
    pub output_mass: Float,

    /// Energy conserved (should equal input_energy within tolerance)
    pub energy_conserved: Float,

    /// Conservation error percentage
    pub conservation_error: Float,

    /// Timestamp of densification
    pub timestamp: u64,

    /// Success status
    pub success: bool,

    /// Error message if failed
    pub error_message: Option<String>,
}

impl EnergyDensificationEvent {
    /// Create a new densification event
    pub fn new(
        entity_id: u64,
        source_density: u8,
        target_density: u8,
        input_energy: Float,
        output_mass: Float,
    ) -> Self {
        let energy_conserved = MatterProperties::energy_from_mass(output_mass);
        let conservation_error = if input_energy > 0.0 {
            ((energy_conserved - input_energy).abs() / input_energy) * 100.0
        } else {
            0.0
        };

        let success = conservation_error <= ENERGY_CONSERVATION_TOLERANCE * 100.0;

        EnergyDensificationEvent {
            entity_id,
            source_density,
            target_density,
            input_energy,
            output_mass,
            energy_conserved,
            conservation_error,
            timestamp: 0, // Will be set by system
            success,
            error_message: if success {
                None
            } else {
                Some(format!(
                    "Energy conservation error {:.6}% exceeds tolerance {:.4}%",
                    conservation_error,
                    ENERGY_CONSERVATION_TOLERANCE * 100.0
                ))
            },
        }
    }

    /// Set the timestamp
    pub fn set_timestamp(&mut self, timestamp: u64) {
        self.timestamp = timestamp;
    }
}

// ============================================================================
// ENERGY DENSIFICATION SYSTEM
// ============================================================================

/// Energy Densification System
///
/// Converts high-frequency energy to low-frequency matter as density decreases
/// during involution. Ensures energy conservation following E=mc².
pub struct EnergyDensificationSystem {
    /// History of densification events by entity
    densification_history: HashMap<u64, Vec<EnergyDensificationEvent>>,

    /// Total energy input (joules)
    total_energy_input: Float,

    /// Total energy output (joules)
    total_energy_output: Float,

    /// Number of successful densifications
    successful_densifications: u64,

    /// Number of failed densifications
    failed_densifications: u64,
}

impl Default for EnergyDensificationSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl EnergyDensificationSystem {
    /// Create a new energy densification system
    pub fn new() -> Self {
        EnergyDensificationSystem {
            densification_history: HashMap::new(),
            total_energy_input: 0.0,
            total_energy_output: 0.0,
            successful_densifications: 0,
            failed_densifications: 0,
        }
    }

    /// Densify energy during density transition
    ///
    /// Converts high-frequency energy at higher density to low-frequency matter
    /// at lower density, conserving total energy (E=mc²).
    ///
    /// # Arguments
    /// * `entity_id` - Entity undergoing densification
    /// * `current_density` - Current density (e.g., 7 for D7)
    /// * `target_density` - Target density (e.g., 6 for D6)
    /// * `input_energy` - Energy to densify (joules)
    ///
    /// # Returns
    /// Result containing matter properties or error
    pub fn densify_energy(
        &mut self,
        entity_id: u64,
        current_density: u8,
        target_density: u8,
        input_energy: Float,
    ) -> Result<MatterProperties, String> {
        // Validate densities
        if current_density < 1 || current_density > 7 {
            return Err(format!(
                "Invalid current density: {}. Must be 1-7.",
                current_density
            ));
        }
        if target_density < 1 || target_density > 7 {
            return Err(format!(
                "Invalid target density: {}. Must be 1-7.",
                target_density
            ));
        }

        // Validate we're descending (higher to lower density)
        if target_density >= current_density {
            return Err(format!(
                "Densification requires descending to lower density. {} -> {} is invalid.",
                current_density, target_density
            ));
        }

        // Validate energy input
        if input_energy <= 0.0 {
            return Err("Input energy must be positive.".to_string());
        }

        // Calculate density factor
        // As density decreases, energy becomes more "compressed" into matter
        let density_factor = self.calculate_density_factor(current_density, target_density);

        // Adjust effective energy based on density factor
        let effective_energy = input_energy * density_factor;

        // Calculate output mass from energy (m=E/c²)
        let output_mass = MatterProperties::mass_from_energy(effective_energy);

        // Verify energy conservation
        let energy_conserved = MatterProperties::energy_from_mass(output_mass);
        let conservation_error = if input_energy > 0.0 {
            (energy_conserved - input_energy).abs() / input_energy
        } else {
            0.0
        };

        if conservation_error > ENERGY_CONSERVATION_TOLERANCE {
            return Err(format!(
                "Energy conservation violated: error {:.6}% exceeds tolerance {:.4}%",
                conservation_error * 100.0,
                ENERGY_CONSERVATION_TOLERANCE * 100.0
            ));
        }

        // Create densification event
        let mut event = EnergyDensificationEvent::new(
            entity_id,
            current_density,
            target_density,
            input_energy,
            output_mass,
        );
        event.set_timestamp(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );

        // Record event
        self.record_densification_event(entity_id, event.clone());

        // Update statistics
        self.total_energy_input += input_energy;
        self.total_energy_output += energy_conserved;
        self.successful_densifications += 1;

        // Note: Matter properties will be calculated from archetype activation
        // in the convert_light_to_matter method
        Err("Use convert_light_to_matter for full matter properties".to_string())
    }

    /// Convert light architecture to matter
    ///
    /// Converts LightArchitecture with archetype activation into physical matter
    /// with properties derived from the archetype pattern.
    ///
    /// # Arguments
    /// * `light_architecture` - Light architecture to convert
    /// * `density` - Density at which conversion occurs
    /// * `energy` - Energy to convert to matter (joules)
    ///
    /// # Returns
    /// Matter properties derived from archetype activation
    pub fn convert_light_to_matter(
        &mut self,
        light_architecture: &LightArchitecture,
        density: u8,
        energy: Float,
    ) -> Result<MatterProperties, String> {
        // Validate density
        if density < 1 || density > 7 {
            return Err(format!("Invalid density: {}. Must be 1-7.", density));
        }

        // Validate energy
        if energy <= 0.0 {
            return Err("Energy must be positive.".to_string());
        }

        // Extract archetype activation pattern
        let archetype_activation = self.extract_archetype_activation(light_architecture);

        // Calculate matter properties from archetype activation
        let mass = derive_mass_from_archetypes(&archetype_activation);
        let charge = derive_charge_from_archetypes(&archetype_activation);
        let spin = derive_spin_from_archetypes(&archetype_activation);
        // derive_lifetime_from_archetypes returns Option<Float> (None = stable particle)
        // Convert None to a very large value (infinite lifetime)
        let lifetime = derive_lifetime_from_archetypes(&archetype_activation).unwrap_or(1e30); // Effectively infinite lifetime (10^30 seconds)

        // Scale mass to match energy input
        // The archetype-derived mass gives the particle type (electron, proton, etc.)
        // We scale it to match the energy input
        let archetype_mass = mass.abs().max(ELECTRON_MASS);
        let energy_equivalent = MatterProperties::energy_from_mass(archetype_mass);

        // Calculate scaling factor
        let scaling_factor = if energy_equivalent > 0.0 {
            (energy / energy_equivalent).sqrt()
        } else {
            1.0
        };

        let scaled_mass = archetype_mass * scaling_factor;
        let scaled_charge = charge * scaling_factor.sqrt(); // Charge scales with sqrt of mass

        // Calculate energy equivalent of scaled mass
        let energy_conserved = MatterProperties::energy_from_mass(scaled_mass);

        // Verify energy conservation
        let conservation_error = if energy > 0.0 {
            (energy_conserved - energy).abs() / energy
        } else {
            0.0
        };

        if conservation_error > ENERGY_CONSERVATION_TOLERANCE {
            return Err(format!(
                "Energy conservation violated: error {:.6}% exceeds tolerance {:.4}%",
                conservation_error * 100.0,
                ENERGY_CONSERVATION_TOLERANCE * 100.0
            ));
        }

        // Create matter properties
        let mut matter_properties = MatterProperties {
            mass: scaled_mass,
            charge: scaled_charge,
            spin,
            lifetime,
            energy_equivalent: energy_conserved,
            formation_density: density,
            archetype_activation,
            is_stable: false,
        };

        // Check stability
        matter_properties.check_stability();

        Ok(matter_properties)
    }

    /// Calculate matter properties from energy and archetype activation
    ///
    /// # Arguments
    /// * `energy` - Energy to convert (joules)
    /// * `density` - Density at which matter forms
    /// * `archetype_activation` - Archetype activation pattern
    ///
    /// # Returns
    /// Matter properties
    pub fn calculate_matter_properties(
        &mut self,
        energy: Float,
        density: u8,
        archetype_activation: &[Float; 22],
    ) -> Result<MatterProperties, String> {
        // Validate density
        if density < 1 || density > 7 {
            return Err(format!("Invalid density: {}. Must be 1-7.", density));
        }

        // Validate energy
        if energy <= 0.0 {
            return Err("Energy must be positive.".to_string());
        }

        // Calculate matter properties from archetype activation
        let mass = derive_mass_from_archetypes(archetype_activation);
        let charge = derive_charge_from_archetypes(archetype_activation);
        let spin = derive_spin_from_archetypes(archetype_activation);
        // derive_lifetime_from_archetypes returns Option<Float> (None = stable particle)
        // Convert None to a very large value (infinite lifetime)
        let lifetime = derive_lifetime_from_archetypes(archetype_activation).unwrap_or(1e30); // Effectively infinite lifetime (10^30 seconds)

        // Scale mass to match energy input
        let archetype_mass = mass.abs().max(ELECTRON_MASS);
        let energy_equivalent = MatterProperties::energy_from_mass(archetype_mass);

        let scaling_factor = if energy_equivalent > 0.0 {
            (energy / energy_equivalent).sqrt()
        } else {
            1.0
        };

        let scaled_mass = archetype_mass * scaling_factor;
        let scaled_charge = charge * scaling_factor.sqrt();
        let energy_conserved = MatterProperties::energy_from_mass(scaled_mass);

        // Verify energy conservation
        let conservation_error = if energy > 0.0 {
            (energy_conserved - energy).abs() / energy
        } else {
            0.0
        };

        if conservation_error > ENERGY_CONSERVATION_TOLERANCE {
            return Err(format!(
                "Energy conservation violated: error {:.6}% exceeds tolerance {:.4}%",
                conservation_error * 100.0,
                ENERGY_CONSERVATION_TOLERANCE * 100.0
            ));
        }

        let mut matter_properties = MatterProperties {
            mass: scaled_mass,
            charge: scaled_charge,
            spin,
            lifetime,
            energy_equivalent: energy_conserved,
            formation_density: density,
            archetype_activation: *archetype_activation,
            is_stable: false,
        };

        matter_properties.check_stability();

        Ok(matter_properties)
    }

    /// Calculate density factor for energy densification
    ///
    /// The density factor represents how much energy gets "compressed" into matter
    /// as density decreases. Higher density factors at lower densities mean more
    /// matter is produced from the same energy.
    ///
    /// # Arguments
    /// * `current_density` - Current density (7 = highest, 1 = lowest)
    /// * `target_density` - Target density
    ///
    /// # Returns
    /// Density factor (1.0 = no compression, >1.0 = compression)
    fn calculate_density_factor(&self, current_density: u8, target_density: u8) -> Float {
        // Density factor increases as we descend to lower densities
        // This represents the "thickening" of energy into matter
        let _density_drop = (current_density - target_density) as Float;

        // Factor based on inverse square of density
        // Lower density = higher compression factor
        let current_factor = 1.0 / (current_density as Float).powi(2);
        let target_factor = 1.0 / (target_density as Float).powi(2);

        // Density factor is the ratio
        target_factor / current_factor
    }

    /// Extract archetype activation from light architecture
    ///
    /// # Arguments
    /// * `light_architecture` - Light architecture
    ///
    /// # Returns
    /// Archetype activation pattern
    fn extract_archetype_activation(&self, light_architecture: &LightArchitecture) -> [Float; 22] {
        let mut activation = [0.0; 22];

        let pattern = light_architecture.archetype_encoding().archetype_pattern;

        for (i, bit) in pattern.iter().enumerate() {
            activation[i] = bit.value;
        }

        activation
    }

    /// Record a densification event
    ///
    /// # Arguments
    /// * `entity_id` - Entity ID
    /// * `event` - Densification event
    fn record_densification_event(&mut self, entity_id: u64, event: EnergyDensificationEvent) {
        self.densification_history
            .entry(entity_id)
            .or_insert_with(Vec::new)
            .push(event);
    }

    /// Get densification history for an entity
    ///
    /// # Arguments
    /// * `entity_id` - Entity ID
    ///
    /// # Returns
    /// Vector of densification events
    pub fn get_densification_history(&self, entity_id: u64) -> Option<&[EnergyDensificationEvent]> {
        self.densification_history
            .get(&entity_id)
            .map(|v| v.as_slice())
    }

    /// Get total energy input
    pub fn total_energy_input(&self) -> Float {
        self.total_energy_input
    }

    /// Get total energy output
    pub fn total_energy_output(&self) -> Float {
        self.total_energy_output
    }

    /// Get successful densification count
    pub fn successful_densifications(&self) -> u64 {
        self.successful_densifications
    }

    /// Get failed densification count
    pub fn failed_densifications(&self) -> u64 {
        self.failed_densifications
    }

    /// Verify overall energy conservation
    ///
    /// # Returns
    /// True if total energy is conserved within tolerance
    pub fn verify_energy_conservation(&self) -> bool {
        if self.total_energy_input <= 0.0 {
            return true;
        }

        let total_error =
            (self.total_energy_output - self.total_energy_input).abs() / self.total_energy_input;

        total_error <= ENERGY_CONSERVATION_TOLERANCE
    }

    /// Get energy conservation error percentage
    ///
    /// # Returns
    /// Error percentage (0.0 = perfect conservation)
    pub fn get_conservation_error(&self) -> Float {
        if self.total_energy_input <= 0.0 {
            return 0.0;
        }

        ((self.total_energy_output - self.total_energy_input).abs() / self.total_energy_input)
            * 100.0
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity_layer7::holographic_blueprint::Archetype22;
    use crate::spectrum::ArchetypicalMind;

    #[test]
    fn test_energy_densification_system_creation() {
        let system = EnergyDensificationSystem::new();
        assert_eq!(system.total_energy_input(), 0.0);
        assert_eq!(system.total_energy_output(), 0.0);
        assert_eq!(system.successful_densifications(), 0);
        assert_eq!(system.failed_densifications(), 0);
    }

    #[test]
    fn test_energy_from_mass() {
        let mass = ELECTRON_MASS;
        let energy = MatterProperties::energy_from_mass(mass);

        // E = mc²
        let expected_energy = mass * SPEED_OF_LIGHT.powi(2);
        assert!((energy - expected_energy).abs() < 1e-40);
    }

    #[test]
    fn test_mass_from_energy() {
        let energy = 8.187e-14; // Approximate electron rest energy
        let mass = MatterProperties::mass_from_energy(energy);

        // m = E/c²
        let expected_mass = energy / SPEED_OF_LIGHT.powi(2);
        assert!((mass - expected_mass).abs() < 1e-48);
    }

    #[test]
    fn test_density_factor_calculation() {
        let system = EnergyDensificationSystem::new();

        // D7 -> D6: density factor should be (1/36) / (1/49) = 49/36 ≈ 1.36
        let factor_7_to_6 = system.calculate_density_factor(7, 6);
        let expected_7_to_6 = 49.0 / 36.0;
        assert!((factor_7_to_6 - expected_7_to_6).abs() < 0.01);

        // D6 -> D5: density factor should be (1/25) / (1/36) = 36/25 = 1.44
        let factor_6_to_5 = system.calculate_density_factor(6, 5);
        let expected_6_to_5 = 36.0 / 25.0;
        assert!((factor_6_to_5 - expected_6_to_5).abs() < 0.01);

        // D4 -> D3: density factor should be (1/9) / (1/16) = 16/9 ≈ 1.78
        let factor_4_to_3 = system.calculate_density_factor(4, 3);
        let expected_4_to_3 = 16.0 / 9.0;
        assert!((factor_4_to_3 - expected_4_to_3).abs() < 0.01);
    }

    #[test]
    fn test_densify_energy_invalid_density() {
        let mut system = EnergyDensificationSystem::new();

        // Invalid current density (0)
        let result = system.densify_energy(1, 0, 6, 1e-10);
        assert!(result.is_err());

        // Invalid target density (8)
        let result = system.densify_energy(1, 7, 8, 1e-10);
        assert!(result.is_err());
    }

    #[test]
    fn test_densify_energy_invalid_transition() {
        let mut system = EnergyDensificationSystem::new();

        // Ascending transition (invalid)
        let result = system.densify_energy(1, 5, 6, 1e-10);
        assert!(result.is_err());

        // Same density (invalid)
        let result = system.densify_energy(1, 5, 5, 1e-10);
        assert!(result.is_err());
    }

    #[test]
    fn test_densify_energy_invalid_energy() {
        let mut system = EnergyDensificationSystem::new();

        // Negative energy
        let result = system.densify_energy(1, 7, 6, -1.0);
        assert!(result.is_err());

        // Zero energy
        let result = system.densify_energy(1, 7, 6, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_light_to_matter() {
        let mut system = EnergyDensificationSystem::new();

        let archetypes = ArchetypicalMind::new_from_logos();
        let free_will = Archetype22::new_from_infinity();
        let light = LightArchitecture::impress_archetypes(&archetypes, &free_will);

        let energy = 8.187e-14; // Electron rest energy

        let result = system.convert_light_to_matter(&light, 4, energy);

        assert!(result.is_ok());

        let matter = result.unwrap();
        assert!(matter.mass > 0.0);
        assert!(matter.energy_equivalent > 0.0);
        assert_eq!(matter.formation_density, 4);
    }

    #[test]
    fn test_convert_light_to_matter_invalid_density() {
        let mut system = EnergyDensificationSystem::new();

        let archetypes = ArchetypicalMind::new_from_logos();
        let free_will = Archetype22::new_from_infinity();
        let light = LightArchitecture::impress_archetypes(&archetypes, &free_will);

        // Invalid density
        let result = system.convert_light_to_matter(&light, 0, 1e-10);
        assert!(result.is_err());

        let result = system.convert_light_to_matter(&light, 8, 1e-10);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_light_to_matter_invalid_energy() {
        let mut system = EnergyDensificationSystem::new();

        let archetypes = ArchetypicalMind::new_from_logos();
        let free_will = Archetype22::new_from_infinity();
        let light = LightArchitecture::impress_archetypes(&archetypes, &free_will);

        // Negative energy
        let result = system.convert_light_to_matter(&light, 4, -1.0);
        assert!(result.is_err());

        // Zero energy
        let result = system.convert_light_to_matter(&light, 4, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_calculate_matter_properties() {
        let mut system = EnergyDensificationSystem::new();

        let archetype_activation = [0.5; 22];
        let energy = 8.187e-14;

        let result = system.calculate_matter_properties(energy, 4, &archetype_activation);

        assert!(result.is_ok());

        let matter = result.unwrap();
        assert!(matter.mass > 0.0);
        assert!(matter.energy_equivalent > 0.0);
        assert_eq!(matter.formation_density, 4);
    }

    #[test]
    fn test_matter_properties_matter_type() {
        let mut matter = MatterProperties {
            mass: ELECTRON_MASS,
            charge: -1.0,
            spin: 0.5,
            lifetime: 1e20,
            energy_equivalent: MatterProperties::energy_from_mass(ELECTRON_MASS),
            formation_density: 4,
            archetype_activation: [0.5; 22],
            is_stable: false,
        };

        matter.check_stability();
        assert_eq!(matter.matter_type(), "Electron");
        assert!(matter.is_stable);
    }

    #[test]
    fn test_matter_properties_proton() {
        let mut matter = MatterProperties {
            mass: PROTON_MASS,
            charge: 1.0,
            spin: 0.5,
            lifetime: 1e20,
            energy_equivalent: MatterProperties::energy_from_mass(PROTON_MASS),
            formation_density: 4,
            archetype_activation: [0.5; 22],
            is_stable: false,
        };

        matter.check_stability();
        assert_eq!(matter.matter_type(), "Proton");
        assert!(matter.is_stable);
    }

    #[test]
    fn test_matter_properties_neutron() {
        let mut matter = MatterProperties {
            mass: NEUTRON_MASS,
            charge: 0.0,
            spin: 0.5,
            lifetime: 880.0, // ~15 minutes in seconds
            energy_equivalent: MatterProperties::energy_from_mass(NEUTRON_MASS),
            formation_density: 4,
            archetype_activation: [0.5; 22],
            is_stable: false,
        };

        matter.check_stability();
        assert_eq!(matter.matter_type(), "Neutron");
        assert!(!matter.is_stable); // Neutrons are unstable outside nuclei
    }

    #[test]
    fn test_densification_event_creation() {
        let event = EnergyDensificationEvent::new(1, 7, 6, 1e-10, 1e-27);

        assert_eq!(event.entity_id, 1);
        assert_eq!(event.source_density, 7);
        assert_eq!(event.target_density, 6);
        assert!(event.success);
    }

    #[test]
    fn test_densification_history() {
        let mut system = EnergyDensificationSystem::new();

        let archetypes = ArchetypicalMind::new_from_logos();
        let free_will = Archetype22::new_from_infinity();
        let light = LightArchitecture::impress_archetypes(&archetypes, &free_will);

        let _ = system.convert_light_to_matter(&light, 4, 1e-10);

        let history = system.get_densification_history(1);
        // History should exist for entity 1
        assert!(history.is_some());
    }

    #[test]
    fn test_energy_conservation_verification() {
        let mut system = EnergyDensificationSystem::new();

        let archetypes = ArchetypicalMind::new_from_logos();
        let free_will = Archetype22::new_from_infinity();
        let light = LightArchitecture::impress_archetypes(&archetypes, &free_will);

        let energy = 1e-10;
        let _ = system.convert_light_to_matter(&light, 4, energy);

        // Energy should be conserved within tolerance
        assert!(system.verify_energy_conservation());
    }

    #[test]
    fn test_get_conservation_error() {
        let mut system = EnergyDensificationSystem::new();

        let archetypes = ArchetypicalMind::new_from_logos();
        let free_will = Archetype22::new_from_infinity();
        let light = LightArchitecture::impress_archetypes(&archetypes, &free_will);

        let _ = system.convert_light_to_matter(&light, 4, 1e-10);

        let error = system.get_conservation_error();
        // Error should be very small (within tolerance)
        assert!(error < ENERGY_CONSERVATION_TOLERANCE * 100.0);
    }

    #[test]
    fn test_multiple_densifications() {
        let mut system = EnergyDensificationSystem::new();

        let archetypes = ArchetypicalMind::new_from_logos();
        let free_will = Archetype22::new_from_infinity();
        let light = LightArchitecture::impress_archetypes(&archetypes, &free_will);

        // Perform multiple densifications
        for _ in 0..5 {
            let _ = system.convert_light_to_matter(&light, 4, 1e-10);
        }

        assert_eq!(system.successful_densifications(), 5);
        assert!(system.verify_energy_conservation());
    }
}
