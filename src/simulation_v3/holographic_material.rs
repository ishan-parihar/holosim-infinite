//! Holographic Material System
//!
//! From GAMING_ENGINE_ROADMAP_v2.md Section 8: "Holographic Material System"
//! "Material properties from holographic resonance. Resonance frequency, density signature,
//! spectrum affinity determine material behavior."
//!
//! From COSMOLOGICAL-ARCHITECTURE.md Section 2.5: "The Nature of Space/Time and Time/Space"
//! "Space and time are not independent variables but fundamentally reciprocal aspects
//! of a unified progression expressed through motion."
//!
//! Key Concepts:
//! - Resonance Frequency: Unique frequency derived from holographic signature
//! - Density Signature: Material's position within the density octave
//! - Spectrum Affinity: Material's affinity for space/time vs time/space
//! - Emergent Properties: Material properties emerge from interference patterns

use crate::simulation_v3::holographic_physics::SpectrumRatio;
use crate::types::Float;
use std::collections::HashMap;

pub const BASE_RESONANCE: Float = 1e14;
pub const RESONANCE_MULTIPLIER: Float = 1e12;
pub const DENSITY_MULTIPLIER: Float = 1e6;
pub const MAX_MATERIALS: usize = 10000;
pub const DEFAULT_CACHE_SIZE: usize = 1000;

pub const HARDNESS_MIN: Float = 0.0;
pub const HARDNESS_MAX: Float = 10.0;
pub const CONDUCTIVITY_MIN: Float = 0.0;
pub const CONDUCTIVITY_MAX: Float = 1e8;
pub const REFRACTIVE_INDEX_MIN: Float = 1.0;
pub const REFRACTIVE_INDEX_MAX: Float = 3.0;
pub const THERMAL_CONDUCTIVITY_MIN: Float = 0.01;
pub const THERMAL_CONDUCTIVITY_MAX: Float = 400.0;
pub const DENSITY_MIN: Float = 1.0;
pub const DENSITY_MAX: Float = 25000.0;
pub const ELASTIC_MODULUS_MIN: Float = 1e3;
pub const ELASTIC_MODULUS_MAX: Float = 1e12;
pub const DIELECTRIC_CONSTANT_MIN: Float = 1.0;
pub const DIELECTRIC_CONSTANT_MAX: Float = 1000.0;
pub const MAGNETIC_PERMEABILITY_MIN: Float = 1.0;
pub const MAGNETIC_PERMEABILITY_MAX: Float = 1e6;
pub const SPECIFIC_HEAT_MIN: Float = 100.0;
pub const SPECIFIC_HEAT_MAX: Float = 10000.0;
pub const ELECTRICAL_RESISTIVITY_MIN: Float = 1e-10;
pub const ELECTRICAL_RESISTIVITY_MAX: Float = 1e10;
pub const CHEMICAL_REACTIVITY_MIN: Float = 0.0;
pub const CHEMICAL_REACTIVITY_MAX: Float = 10.0;
pub const BIOCOMPATIBILITY_MIN: Float = 0.0;
pub const BIOCOMPATIBILITY_MAX: Float = 1.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaterialId(pub u64);

impl MaterialId {
    pub fn new(id: u64) -> Self {
        MaterialId(id)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

impl Default for MaterialId {
    fn default() -> Self {
        MaterialId(0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MaterialProperties {
    pub hardness: Float,
    pub conductivity: Float,
    pub refractive_index: Float,
    pub thermal_conductivity: Float,
    pub mass_density: Float,
    pub elastic_modulus: Float,
    pub dielectric_constant: Float,
    pub magnetic_permeability: Float,
    pub specific_heat: Float,
    pub electrical_resistivity: Float,
    pub chemical_reactivity: Float,
    pub biocompatibility: Float,
    pub spectral_absorption: [Float; 10],
    pub spectral_emission: [Float; 10],
}

impl Default for MaterialProperties {
    fn default() -> Self {
        MaterialProperties {
            hardness: 1.0,
            conductivity: 0.01,
            refractive_index: 1.5,
            thermal_conductivity: 0.1,
            mass_density: 1000.0,
            elastic_modulus: 1e9,
            dielectric_constant: 4.0,
            magnetic_permeability: 1.0,
            specific_heat: 1000.0,
            electrical_resistivity: 100.0,
            chemical_reactivity: 1.0,
            biocompatibility: 0.5,
            spectral_absorption: [0.1; 10],
            spectral_emission: [0.05; 10],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HolographicMaterial {
    pub material_id: MaterialId,
    pub resonance_frequency: Float,
    pub density_signature: Float,
    pub spectrum_affinity: SpectrumRatio,
    pub holographic_signature: [Float; 22],
    pub base_properties: MaterialProperties,
    pub phase_state: MaterialPhase,
    pub temperature: Float,
    pub pressure: Float,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaterialPhase {
    Solid,
    Liquid,
    Gas,
    Plasma,
    BoseEinsteinCondensate,
    DegenerateMatter,
    Neutronium,
    Photonic,
    Holographic,
    Quantum,
}

impl Default for MaterialPhase {
    fn default() -> Self {
        MaterialPhase::Solid
    }
}

#[derive(Debug, Clone)]
pub struct ResonanceCache {
    cache: HashMap<ResonanceKey, Float>,
    max_size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResonanceKey {
    pub signature_hash: u64,
    pub density_level: u8,
}

#[derive(Debug, Clone)]
pub enum HolographicMaterialError {
    InvalidSignature(String),
    InvalidResonance(String),
    MaterialNotFound(MaterialId),
    CacheFull,
    ComputationFailed(String),
    PropertyOutOfRange(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MaterialSystemStatistics {
    pub total_materials: usize,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub total_resonance_computations: u64,
    pub average_computation_time_ns: u64,
    pub materials_by_phase: HashMap<MaterialPhase, usize>,
}

impl Default for MaterialSystemStatistics {
    fn default() -> Self {
        MaterialSystemStatistics {
            total_materials: 0,
            cache_hits: 0,
            cache_misses: 0,
            total_resonance_computations: 0,
            average_computation_time_ns: 0,
            materials_by_phase: HashMap::new(),
        }
    }
}

pub struct HolographicMaterialSystem {
    materials: HashMap<MaterialId, HolographicMaterial>,
    resonance_cache: ResonanceCache,
    next_material_id: u64,
    statistics: MaterialSystemStatistics,
}

impl Default for HolographicMaterialSystem {
    fn default() -> Self {
        HolographicMaterialSystem::new()
    }
}

impl HolographicMaterialSystem {
    pub fn new() -> Self {
        HolographicMaterialSystem {
            materials: HashMap::new(),
            resonance_cache: ResonanceCache::new(DEFAULT_CACHE_SIZE),
            next_material_id: 1,
            statistics: MaterialSystemStatistics::default(),
        }
    }

    pub fn create_material(
        &mut self,
        signature: [Float; 22],
    ) -> Result<HolographicMaterial, HolographicMaterialError> {
        self.validate_signature(&signature)?;

        let material_id = MaterialId::new(self.next_material_id);
        self.next_material_id += 1;

        let resonance_frequency = self.compute_resonance_frequency(&signature)?;
        let density_signature =
            self.compute_density_signature_internal(&signature, resonance_frequency);
        let spectrum_affinity =
            self.compute_spectrum_affinity_internal(&signature, resonance_frequency);
        let base_properties = self.compute_material_properties_internal(
            &signature,
            resonance_frequency,
            density_signature,
        );
        let phase_state = self.determine_initial_phase(&base_properties, 300.0);

        let material = HolographicMaterial {
            material_id,
            resonance_frequency,
            density_signature,
            spectrum_affinity,
            holographic_signature: signature,
            base_properties,
            phase_state,
            temperature: 300.0,
            pressure: 101325.0,
        };

        self.materials.insert(material_id, material.clone());
        self.statistics.total_materials += 1;
        *self
            .statistics
            .materials_by_phase
            .entry(phase_state)
            .or_insert(0) += 1;

        Ok(material)
    }

    pub fn get_material(&self, material_id: MaterialId) -> Option<&HolographicMaterial> {
        self.materials.get(&material_id)
    }

    pub fn update_material_temperature(
        &mut self,
        material_id: MaterialId,
        new_temperature: Float,
    ) -> Result<(), HolographicMaterialError> {
        let old_phase = self
            .materials
            .get(&material_id)
            .ok_or_else(|| HolographicMaterialError::MaterialNotFound(material_id))?
            .phase_state;

        let base_properties = self
            .materials
            .get(&material_id)
            .ok_or_else(|| HolographicMaterialError::MaterialNotFound(material_id))?
            .base_properties
            .clone();

        let pressure = self
            .materials
            .get(&material_id)
            .ok_or_else(|| HolographicMaterialError::MaterialNotFound(material_id))?
            .pressure;

        let new_phase = self.determine_phase(&base_properties, new_temperature, pressure);

        let material = self
            .materials
            .get_mut(&material_id)
            .ok_or_else(|| HolographicMaterialError::MaterialNotFound(material_id))?;

        material.temperature = new_temperature;
        material.phase_state = new_phase;

        if old_phase != new_phase {
            *self
                .statistics
                .materials_by_phase
                .entry(old_phase)
                .or_insert(0) -= 1;
            *self
                .statistics
                .materials_by_phase
                .entry(new_phase)
                .or_insert(0) += 1;
        }

        Ok(())
    }

    pub fn update_material_pressure(
        &mut self,
        material_id: MaterialId,
        new_pressure: Float,
    ) -> Result<(), HolographicMaterialError> {
        let old_phase = self
            .materials
            .get(&material_id)
            .ok_or_else(|| HolographicMaterialError::MaterialNotFound(material_id))?
            .phase_state;

        let base_properties = self
            .materials
            .get(&material_id)
            .ok_or_else(|| HolographicMaterialError::MaterialNotFound(material_id))?
            .base_properties
            .clone();

        let temperature = self
            .materials
            .get(&material_id)
            .ok_or_else(|| HolographicMaterialError::MaterialNotFound(material_id))?
            .temperature;

        let new_phase = self.determine_phase(&base_properties, temperature, new_pressure);

        let material = self
            .materials
            .get_mut(&material_id)
            .ok_or_else(|| HolographicMaterialError::MaterialNotFound(material_id))?;

        material.pressure = new_pressure;
        material.phase_state = new_phase;

        if old_phase != new_phase {
            *self
                .statistics
                .materials_by_phase
                .entry(old_phase)
                .or_insert(0) -= 1;
            *self
                .statistics
                .materials_by_phase
                .entry(new_phase)
                .or_insert(0) += 1;
        }

        Ok(())
    }

    pub fn compute_resonance_frequency(
        &mut self,
        signature: &[Float; 22],
    ) -> Result<Float, HolographicMaterialError> {
        let key = ResonanceKey {
            signature_hash: self.signature_hash(signature),
            density_level: 3,
        };

        if let Some(frequency) = self.resonance_cache.get(&key) {
            self.statistics.cache_hits += 1;
            return Ok(frequency);
        }

        self.statistics.cache_misses += 1;
        self.statistics.total_resonance_computations += 1;

        let resonance = self.compute_resonance_frequency_internal(signature)?;
        self.resonance_cache.insert(key, resonance);

        Ok(resonance)
    }

    fn compute_resonance_frequency_internal(
        &self,
        signature: &[Float; 22],
    ) -> Result<Float, HolographicMaterialError> {
        let sum: Float = signature.iter().sum();
        let avg = sum / 22.0;
        let variance: Float = signature.iter().map(|&v| (v - avg).powi(2)).sum::<Float>() / 22.0;
        let std_dev = variance.sqrt();

        let primary_component: Float = signature
            .iter()
            .fold(0.0, |acc: Float, &v| acc.max(v.abs()));
        let interference_pattern = signature
            .iter()
            .enumerate()
            .map(|(i, &v)| v * ((i as Float * 3.14159) / 11.0).sin())
            .sum::<Float>()
            / 22.0;

        let resonance = BASE_RESONANCE
            * (1.0 + primary_component * RESONANCE_MULTIPLIER)
            * (1.0 + std_dev * 0.1)
            * (1.0 + interference_pattern.abs() * 0.5);

        Ok(resonance)
    }

    fn compute_density_signature_internal(
        &self,
        signature: &[Float; 22],
        resonance_frequency: Float,
    ) -> Float {
        let low_density_weight: Float = signature[0..7].iter().sum::<Float>() / 7.0;
        let mid_density_weight: Float = signature[7..15].iter().sum::<Float>() / 8.0;
        let high_density_weight: Float = signature[15..22].iter().sum::<Float>() / 7.0;

        let density_position =
            (low_density_weight * 1.0 + mid_density_weight * 4.0 + high_density_weight * 7.0) / 3.0;

        let resonance_factor = (resonance_frequency / BASE_RESONANCE).log10();

        let density_signature = (density_position + resonance_factor) * DENSITY_MULTIPLIER;

        density_signature.clamp(DENSITY_MIN, DENSITY_MAX)
    }

    fn compute_spectrum_affinity_internal(
        &self,
        signature: &[Float; 22],
        resonance_frequency: Float,
    ) -> SpectrumRatio {
        let space_time_components: Float = signature[0..11].iter().sum::<Float>();
        let time_space_components: Float = signature[11..22].iter().sum::<Float>();

        let space_time_ratio = space_time_components.abs()
            / (space_time_components.abs() + time_space_components.abs() + 1e-10);
        let time_space_ratio = time_space_components.abs()
            / (space_time_components.abs() + time_space_components.abs() + 1e-10);

        let resonance_factor = (resonance_frequency / BASE_RESONANCE).ln().tanh();

        let adjusted_st_ratio = space_time_ratio * (1.0 + resonance_factor * 0.5);
        let adjusted_ts_ratio = time_space_ratio * (1.0 - resonance_factor * 0.5);

        SpectrumRatio::new(adjusted_st_ratio, adjusted_ts_ratio)
    }

    fn compute_material_properties_internal(
        &self,
        signature: &[Float; 22],
        resonance_frequency: Float,
        density_signature: Float,
    ) -> MaterialProperties {
        let hardness = self.clamp_property(
            signature[0] * 10.0 * (density_signature / DENSITY_MIN).ln(),
            HARDNESS_MIN,
            HARDNESS_MAX,
        );

        let conductivity = self.clamp_property(
            signature[1] * CONDUCTIVITY_MAX * (resonance_frequency / BASE_RESONANCE).log10(),
            CONDUCTIVITY_MIN,
            CONDUCTIVITY_MAX,
        );

        let refractive_index = self.clamp_property(
            1.0 + signature[2] * 2.0 * (density_signature / DENSITY_MAX).sqrt(),
            REFRACTIVE_INDEX_MIN,
            REFRACTIVE_INDEX_MAX,
        );

        let thermal_conductivity = self.clamp_property(
            signature[3]
                * THERMAL_CONDUCTIVITY_MAX
                * (resonance_frequency / BASE_RESONANCE).powf(0.3),
            THERMAL_CONDUCTIVITY_MIN,
            THERMAL_CONDUCTIVITY_MAX,
        );

        let mass_density = density_signature.clamp(DENSITY_MIN, DENSITY_MAX);

        let elastic_modulus = self.clamp_property(
            signature[4] * ELASTIC_MODULUS_MAX * (mass_density / 1000.0).powf(1.5),
            ELASTIC_MODULUS_MIN,
            ELASTIC_MODULUS_MAX,
        );

        let dielectric_constant = self.clamp_property(
            1.0 + signature[5] * 999.0 * (refractive_index - 1.0),
            DIELECTRIC_CONSTANT_MIN,
            DIELECTRIC_CONSTANT_MAX,
        );

        let magnetic_permeability = self.clamp_property(
            1.0 + signature[6] * (MAGNETIC_PERMEABILITY_MAX - 1.0) * conductivity.abs().log10(),
            MAGNETIC_PERMEABILITY_MIN,
            MAGNETIC_PERMEABILITY_MAX,
        );

        let specific_heat = self.clamp_property(
            SPECIFIC_HEAT_MIN
                + signature[7]
                    * (SPECIFIC_HEAT_MAX - SPECIFIC_HEAT_MIN)
                    * (mass_density / 10000.0).powf(-0.5),
            SPECIFIC_HEAT_MIN,
            SPECIFIC_HEAT_MAX,
        );

        let electrical_resistivity = self.clamp_property(
            ELECTRICAL_RESISTIVITY_MIN
                + signature[8] * (ELECTRICAL_RESISTIVITY_MAX - ELECTRICAL_RESISTIVITY_MIN)
                    / (conductivity + 1e-10),
            ELECTRICAL_RESISTIVITY_MIN,
            ELECTRICAL_RESISTIVITY_MAX,
        );

        let chemical_reactivity = self.clamp_property(
            signature[9] * 10.0 * (1.0 - hardness / HARDNESS_MAX),
            CHEMICAL_REACTIVITY_MIN,
            CHEMICAL_REACTIVITY_MAX,
        );

        let biocompatibility = self.clamp_property(
            signature[10] * (1.0 - chemical_reactivity / 10.0),
            BIOCOMPATIBILITY_MIN,
            BIOCOMPATIBILITY_MAX,
        );

        let spectral_absorption = [
            signature[11],
            signature[12],
            signature[13],
            signature[14],
            signature[15],
            signature[16],
            signature[17],
            signature[18],
            signature[19],
            signature[20],
        ]
        .map(|v| v.abs().clamp(0.0, 1.0));

        let spectral_emission = [
            signature[12],
            signature[13],
            signature[14],
            signature[15],
            signature[16],
            signature[17],
            signature[18],
            signature[19],
            signature[20],
            signature[21],
        ]
        .map(|v| v.abs().clamp(0.0, 1.0) * 0.5);

        MaterialProperties {
            hardness,
            conductivity,
            refractive_index,
            thermal_conductivity,
            mass_density,
            elastic_modulus,
            dielectric_constant,
            magnetic_permeability,
            specific_heat,
            electrical_resistivity,
            chemical_reactivity,
            biocompatibility,
            spectral_absorption,
            spectral_emission,
        }
    }

    pub fn compute_density_signature(
        &self,
        signature: &[Float; 22],
        resonance_frequency: Float,
    ) -> Float {
        self.compute_density_signature_internal(signature, resonance_frequency)
    }

    pub fn compute_spectrum_affinity(
        &self,
        signature: &[Float; 22],
        resonance_frequency: Float,
    ) -> SpectrumRatio {
        self.compute_spectrum_affinity_internal(signature, resonance_frequency)
    }

    pub fn compute_material_properties(
        &self,
        signature: &[Float; 22],
        resonance_frequency: Float,
        density_signature: Float,
    ) -> MaterialProperties {
        self.compute_material_properties_internal(signature, resonance_frequency, density_signature)
    }

    pub fn determine_initial_phase(
        &self,
        properties: &MaterialProperties,
        temperature: Float,
    ) -> MaterialPhase {
        self.determine_phase(properties, temperature, 101325.0)
    }

    pub fn determine_phase(
        &self,
        properties: &MaterialProperties,
        temperature: Float,
        pressure: Float,
    ) -> MaterialPhase {
        let melting_point = properties.hardness * 500.0 + 100.0;
        let boiling_point = melting_point * 2.5;
        let plasma_point = boiling_point * 3.0;
        let bec_point = properties.conductivity * 1e-7;

        let pressure_factor = pressure / 101325.0;
        let adjusted_melting_point = melting_point * pressure_factor.powf(0.8);
        let adjusted_boiling_point = boiling_point * pressure_factor.powf(0.9);

        if temperature < bec_point {
            MaterialPhase::BoseEinsteinCondensate
        } else if temperature < adjusted_melting_point {
            if properties.mass_density > 1e5 {
                MaterialPhase::Neutronium
            } else if properties.mass_density > 1e4 {
                MaterialPhase::DegenerateMatter
            } else {
                MaterialPhase::Solid
            }
        } else if temperature < adjusted_boiling_point {
            MaterialPhase::Liquid
        } else if temperature < plasma_point {
            MaterialPhase::Gas
        } else if properties.conductivity > 1e6 {
            MaterialPhase::Plasma
        } else if properties.refractive_index > 2.5 {
            MaterialPhase::Holographic
        } else {
            MaterialPhase::Quantum
        }
    }

    pub fn get_statistics(&self) -> &MaterialSystemStatistics {
        &self.statistics
    }

    pub fn clear_cache(&mut self) {
        self.resonance_cache.clear();
    }

    fn validate_signature(&self, signature: &[Float; 22]) -> Result<(), HolographicMaterialError> {
        for &value in signature {
            if !value.is_finite() {
                return Err(HolographicMaterialError::InvalidSignature(format!(
                    "Signature contains non-finite value: {}",
                    value
                )));
            }
        }
        Ok(())
    }

    fn signature_hash(&self, signature: &[Float; 22]) -> u64 {
        let mut hash: u64 = 0;
        for (i, &value) in signature.iter().enumerate() {
            let bits =
                (value.to_bits() as u64).wrapping_add((i as u64).wrapping_mul(0x9e3779b97f4a7c15));
            hash = hash.wrapping_add(bits);
            hash = hash.wrapping_mul(0x100000001b3);
            hash ^= hash >> 31;
        }
        hash
    }

    fn clamp_property(&self, value: Float, min: Float, max: Float) -> Float {
        value.clamp(min, max)
    }
}

impl ResonanceCache {
    pub fn new(max_size: usize) -> Self {
        ResonanceCache {
            cache: HashMap::new(),
            max_size,
        }
    }

    pub fn get(&self, key: &ResonanceKey) -> Option<Float> {
        self.cache.get(key).copied()
    }

    pub fn insert(&mut self, key: ResonanceKey, value: Float) {
        if self.cache.len() >= self.max_size {
            self.cache.clear();
        }
        self.cache.insert(key, value);
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }

    pub fn size(&self) -> usize {
        self.cache.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_holographic_material_system_creation() {
        let system = HolographicMaterialSystem::new();
        assert_eq!(system.get_statistics().total_materials, 0);
    }

    #[test]
    fn test_create_material() {
        let mut system = HolographicMaterialSystem::new();
        let signature = [
            0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 0.5, 0.4, 0.3, 0.2, 0.1, 0.6, 0.7,
            0.8, 0.9, 0.1, 0.2, 0.3,
        ];

        let material = system.create_material(signature).unwrap();
        assert_eq!(material.material_id, MaterialId::new(1));
        assert!(material.resonance_frequency > 0.0);
        assert!(material.density_signature > 0.0);
    }

    #[test]
    fn test_resonance_frequency_computation() {
        let mut system = HolographicMaterialSystem::new();
        let signature = [0.1; 22];

        let frequency = system.compute_resonance_frequency(&signature).unwrap();
        assert!(frequency > BASE_RESONANCE);
        assert!(frequency.is_finite());
    }

    #[test]
    fn test_density_signature_computation() {
        let system = HolographicMaterialSystem::new();
        let signature = [0.1; 22];
        let resonance_frequency = BASE_RESONANCE * 2.0;

        let density_sig = system.compute_density_signature(&signature, resonance_frequency);
        assert!(density_sig >= DENSITY_MIN);
        assert!(density_sig <= DENSITY_MAX);
    }

    #[test]
    fn test_spectrum_affinity_computation() {
        let system = HolographicMaterialSystem::new();
        let signature = [0.1; 22];
        let resonance_frequency = BASE_RESONANCE;

        let affinity = system.compute_spectrum_affinity(&signature, resonance_frequency);
        assert!(affinity.space_time_ratio >= 0.0);
        assert!(affinity.time_space_ratio >= 0.0);
    }

    #[test]
    fn test_material_properties_computation() {
        let system = HolographicMaterialSystem::new();
        let signature = [0.1; 22];
        let resonance_frequency = BASE_RESONANCE;
        let density_signature = 1000.0;

        let properties =
            system.compute_material_properties(&signature, resonance_frequency, density_signature);

        assert!(properties.hardness >= HARDNESS_MIN);
        assert!(properties.hardness <= HARDNESS_MAX);
        assert!(properties.conductivity >= CONDUCTIVITY_MIN);
        assert!(properties.conductivity <= CONDUCTIVITY_MAX);
        assert!(properties.refractive_index >= REFRACTIVE_INDEX_MIN);
        assert!(properties.refractive_index <= REFRACTIVE_INDEX_MAX);
    }

    #[test]
    fn test_determine_initial_phase() {
        let system = HolographicMaterialSystem::new();
        let properties = MaterialProperties::default();

        let phase = system.determine_initial_phase(&properties, 300.0);
        assert_eq!(phase, MaterialPhase::Solid);
    }

    #[test]
    fn test_determine_phase_temperature() {
        let system = HolographicMaterialSystem::new();
        let properties = MaterialProperties::default();

        let solid_phase = system.determine_phase(&properties, 200.0, 101325.0);
        assert_eq!(solid_phase, MaterialPhase::Solid);

        let liquid_phase = system.determine_phase(&properties, 600.0, 101325.0);
        assert_eq!(liquid_phase, MaterialPhase::Liquid);

        let gas_phase = system.determine_phase(&properties, 2000.0, 101325.0);
        assert_eq!(gas_phase, MaterialPhase::Gas);
    }

    #[test]
    fn test_determine_phase_pressure() {
        let system = HolographicMaterialSystem::new();
        let mut properties = MaterialProperties::default();
        properties.hardness = 5.0;

        let high_pressure_phase = system.determine_phase(&properties, 600.0, 1e7);
        assert_eq!(high_pressure_phase, MaterialPhase::Solid);

        let low_pressure_phase = system.determine_phase(&properties, 600.0, 1e4);
        assert_eq!(low_pressure_phase, MaterialPhase::Liquid);
    }

    #[test]
    fn test_neutronium_phase() {
        let system = HolographicMaterialSystem::new();
        let mut properties = MaterialProperties::default();
        properties.mass_density = 2e5;
        properties.hardness = 10.0;

        let phase = system.determine_phase(&properties, 100.0, 101325.0);
        assert_eq!(phase, MaterialPhase::Neutronium);
    }

    #[test]
    fn test_plasma_phase() {
        let system = HolographicMaterialSystem::new();
        let mut properties = MaterialProperties::default();
        properties.hardness = 2.0;
        properties.conductivity = 1e7;

        let phase = system.determine_phase(&properties, 10000.0, 101325.0);
        assert_eq!(phase, MaterialPhase::Plasma);
    }

    #[test]
    fn test_holographic_phase() {
        let system = HolographicMaterialSystem::new();
        let mut properties = MaterialProperties::default();
        properties.refractive_index = 3.0;
        properties.hardness = 2.0; // Lower hardness to lower melting/boiling points

        let phase = system.determine_phase(&properties, 10000.0, 101325.0);
        assert_eq!(phase, MaterialPhase::Holographic);
    }

    #[test]
    fn test_update_material_temperature() {
        let mut system = HolographicMaterialSystem::new();
        let signature = [0.1; 22];
        let material = system.create_material(signature).unwrap();
        let material_id = material.material_id;

        system
            .update_material_temperature(material_id, 1000.0)
            .unwrap();
        let updated = system.get_material(material_id).unwrap();
        assert_eq!(updated.temperature, 1000.0);
    }

    #[test]
    fn test_update_material_pressure() {
        let mut system = HolographicMaterialSystem::new();
        let signature = [0.1; 22];
        let material = system.create_material(signature).unwrap();
        let material_id = material.material_id;

        system.update_material_pressure(material_id, 1e6).unwrap();
        let updated = system.get_material(material_id).unwrap();
        assert_eq!(updated.pressure, 1e6);
    }

    #[test]
    fn test_phase_transition() {
        let mut system = HolographicMaterialSystem::new();
        let signature = [0.1; 22];
        let material = system.create_material(signature).unwrap();
        let material_id = material.material_id;
        let initial_phase = material.phase_state;

        system
            .update_material_temperature(material_id, 50000.0)
            .unwrap();
        let updated = system.get_material(material_id).unwrap();
        assert_ne!(initial_phase, updated.phase_state);
    }

    #[test]
    fn test_invalid_signature() {
        let mut system = HolographicMaterialSystem::new();
        let mut signature = [0.1; 22];
        signature[5] = Float::NAN;

        let result = system.create_material(signature);
        assert!(result.is_err());
    }

    #[test]
    fn test_resonance_cache() {
        let mut system = HolographicMaterialSystem::new();
        let signature = [0.5; 22];

        let freq1 = system.compute_resonance_frequency(&signature).unwrap();
        let freq2 = system.compute_resonance_frequency(&signature).unwrap();

        assert_eq!(freq1, freq2);
        assert_eq!(system.get_statistics().cache_hits, 1);
        assert_eq!(system.get_statistics().cache_misses, 1);
    }

    #[test]
    fn test_clear_cache() {
        let mut system = HolographicMaterialSystem::new();
        let signature = [0.5; 22];
        system.compute_resonance_frequency(&signature).unwrap();

        system.clear_cache();

        system.compute_resonance_frequency(&signature).unwrap();
        assert_eq!(system.get_statistics().cache_misses, 2);
    }

    #[test]
    fn test_get_nonexistent_material() {
        let system = HolographicMaterialSystem::new();
        let result = system.get_material(MaterialId::new(999));
        assert!(result.is_none());
    }

    #[test]
    fn test_spectral_properties() {
        let mut system = HolographicMaterialSystem::new();
        let signature = [
            0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 0.5, 0.4, 0.3, 0.2, 0.1, 0.6, 0.7,
            0.8, 0.9, 0.1, 0.2, 0.3,
        ];
        let material = system.create_material(signature).unwrap();

        for absorption in material.base_properties.spectral_absorption {
            assert!(absorption >= 0.0);
            assert!(absorption <= 1.0);
        }

        for emission in material.base_properties.spectral_emission {
            assert!(emission >= 0.0);
            assert!(emission <= 1.0);
        }
    }
}
