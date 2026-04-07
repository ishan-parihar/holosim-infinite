//! Energy Flow System: Star → Planet → Biology → Entity
//!
//! This module implements the complete energy pipeline from stellar radiation
//! through planetary systems to biological productivity and entity availability.
//!
//! From V4 Roadmap Gap #14: "No energy economy"
//!
//! Energy flows:
//! 1. Stellar output (luminosity)
//! 2. Top-of-atmosphere irradiance
//! 3. Surface irradiance (after albedo + atmosphere)
//! 4. Photosynthetic capture
//! 5. Primary productivity
//! 6. Food web transfer
//! 7. Entity-available energy

use crate::cosmos::stellar_physics::Star;

// ============================================================================
// Constants
// ============================================================================

/// Solar constant at 1 AU (W/m²)
/// Note: Used for stellar physics calculations and habitability scoring
const SOLAR_CONSTANT: f64 = 1361.0;

/// Stefan-Boltzmann constant (W/m²/K⁴)
/// Note: Reserved for future black-body radiation calculations
const STEFAN_BOLTZMANN: f64 = 5.67e-8;

/// AU in meters
const AU_TO_METERS: f64 = 1.496e11;

/// Earth average albedo
const EARTH_ALBEDO: f64 = 0.30;

/// Photosynthetic efficiency (fraction of sunlight converted to biomass)
const PHOTOSYNTHETIC_EFFICIENCY: f64 = 0.01;

/// Primary productivity (gC/m²/year) - Earth average
/// Note: Used for biosphere energy calculations
const EARTH_NPP: f64 = 400.0;

/// Trophic efficiency (10% per level)
const TROPHIC_EFFICIENCY: f64 = 0.10;

/// Average trophic levels
const AVG_TROPHIC_LEVELS: f64 = 2.0;

/// Geothermal heat flow (W/m²)
const GEOTHERMAL_FLOW: f64 = 0.087;

// ============================================================================
// Data Structures
// ============================================================================

/// Energy flow metrics at each stage
#[derive(Debug, Clone, Default)]
pub struct EnergyFlowMetrics {
    /// Top-of-atmosphere solar input (W/m²)
    pub stellar_input: f64,
    /// Reflected by albedo (W/m²)
    pub albedo_loss: f64,
    /// Absorbed by atmosphere (W/m²)
    pub atmospheric_absorption: f64,
    /// Reaching surface (W/m²)
    pub surface_irradiance: f64,
    /// Photosynthetically active radiation (W/m²)
    pub par: f64,
    /// Captured by photosynthesis (W/m²)
    pub photosynthetic_capture: f64,
    /// Net primary productivity (W/m²)
    pub npp: f64,
    /// Secondary productivity (W/m²)
    pub secondary_productivity: f64,
    /// Available to apex predators/entity (W/m²)
    pub entity_available: f64,
    /// Geothermal input (W/m²)
    pub geothermal: f64,
    /// Total energy budget (W/m²)
    pub total_input: f64,
}

/// Geographic distribution of energy
#[derive(Debug, Clone)]
pub struct GeographicEnergyDistribution {
    /// Equatorial (0-30°) - W/m²
    pub equatorial: f64,
    /// Temperate (30-60°) - W/m²
    pub temperate: f64,
    /// Polar (60-90°) - W/m²
    pub polar: f64,
    /// Land vs ocean distribution
    pub land_fraction: f64,
    /// Productive area fraction
    pub productive_fraction: f64,
}

/// Seasonal energy variation
#[derive(Debug, Clone)]
pub struct SeasonalVariation {
    /// Summer hemisphere excess (W/m²)
    pub summer_excess: f64,
    /// Winter hemisphere deficit (W/m²)
    pub winter_deficit: f64,
    /// Annual energy throughput (J)
    pub annual_throughput: f64,
    /// Seasonal amplitude (ratio)
    pub seasonal_amplitude: f64,
}

/// Biological productivity zones
#[derive(Debug, Clone)]
pub struct ProductivityZone {
    /// Zone type
    pub zone_type: ProductivityZoneType,
    /// Area (km²)
    pub area: f64,
    /// NPP (gC/m²/year)
    pub npp: f64,
    /// Energy capture (W)
    pub energy_capture: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProductivityZoneType {
    /// High-latitude tundra
    Tundra,
    /// Boreal forests
    Boreal,
    /// Temperate forests
    TemperateForest,
    /// Temperate grasslands
    TemperateGrassland,
    /// Tropical rainforests
    TropicalRainforest,
    /// Tropical savannas
    TropicalSavanna,
    /// Deserts
    Desert,
    /// Oceans - upwelling zones
    CoastalUpwelling,
    /// Open ocean
    OpenOcean,
    /// Polar oceans
    PolarOcean,
}

// ============================================================================
// Main Energy Flow System
// ============================================================================

/// Energy flow system: tracks energy from stellar source through planetary
/// systems to entity availability
#[derive(Debug, Clone)]
pub struct EnergyFlowSystem {
    /// Current energy flow metrics
    pub metrics: EnergyFlowMetrics,
    /// Geographic distribution
    pub geographic: GeographicEnergyDistribution,
    /// Seasonal variation
    pub seasonal: SeasonalVariation,
    /// Productivity zones
    pub productivity_zones: Vec<ProductivityZone>,
    /// Star luminosity (solar luminosities)
    star_luminosity: f64,
    /// Planet orbital radius (AU)
    orbital_radius: f64,
    /// Planet albedo
    planet_albedo: f64,
    /// Atmosphere transmittance
    atmosphere_transmittance: f64,
    /// Photosynthetic efficiency
    photosynthetic_efficiency: f64,
    /// Biosphere present
    biosphere_present: f64,
}

impl Default for EnergyFlowSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl EnergyFlowSystem {
    /// Create new energy flow system
    pub fn new() -> Self {
        Self {
            metrics: EnergyFlowMetrics::default(),
            geographic: GeographicEnergyDistribution {
                equatorial: 0.0,
                temperate: 0.0,
                polar: 0.0,
                land_fraction: 0.29, // Earth
                productive_fraction: 0.5,
            },
            seasonal: SeasonalVariation {
                summer_excess: 0.0,
                winter_deficit: 0.0,
                annual_throughput: 0.0,
                seasonal_amplitude: 0.0,
            },
            productivity_zones: Vec::new(),
            star_luminosity: 1.0,
            orbital_radius: 1.0,
            planet_albedo: EARTH_ALBEDO,
            atmosphere_transmittance: 0.77, // Earth atmosphere transmits ~77%
            photosynthetic_efficiency: PHOTOSYNTHETIC_EFFICIENCY,
            biosphere_present: 1.0,
        }
    }

    /// Initialize with star and planet parameters
    pub fn initialize(
        &mut self,
        star_luminosity: f64,
        orbital_radius: f64,
        albedo: f64,
        atmosphere_transmittance: f64,
    ) {
        self.star_luminosity = star_luminosity;
        self.orbital_radius = orbital_radius;
        self.planet_albedo = albedo;
        self.atmosphere_transmittance = atmosphere_transmittance;

        // Initialize productivity zones
        self.initialize_productivity_zones();
    }

    /// Initialize productivity zones
    fn initialize_productivity_zones(&mut self) {
        self.productivity_zones = vec![
            ProductivityZone {
                zone_type: ProductivityZoneType::TropicalRainforest,
                area: 1.7e7, // 17 million km²
                npp: 900.0,
                energy_capture: 0.0,
            },
            ProductivityZone {
                zone_type: ProductivityZoneType::TemperateForest,
                area: 1.04e8,
                npp: 600.0,
                energy_capture: 0.0,
            },
            ProductivityZone {
                zone_type: ProductivityZoneType::Boreal,
                area: 1.5e7,
                npp: 350.0,
                energy_capture: 0.0,
            },
            ProductivityZone {
                zone_type: ProductivityZoneType::TemperateGrassland,
                area: 9.0e6,
                npp: 250.0,
                energy_capture: 0.0,
            },
            ProductivityZone {
                zone_type: ProductivityZoneType::TropicalSavanna,
                area: 2.0e7,
                npp: 400.0,
                energy_capture: 0.0,
            },
            ProductivityZone {
                zone_type: ProductivityZoneType::Desert,
                area: 3.35e7,
                npp: 30.0,
                energy_capture: 0.0,
            },
            ProductivityZone {
                zone_type: ProductivityZoneType::Tundra,
                area: 8.0e6,
                npp: 50.0,
                energy_capture: 0.0,
            },
            ProductivityZone {
                zone_type: ProductivityZoneType::OpenOcean,
                area: 3.32e8,
                npp: 125.0,
                energy_capture: 0.0,
            },
            ProductivityZone {
                zone_type: ProductivityZoneType::CoastalUpwelling,
                area: 2.2e7,
                npp: 300.0,
                energy_capture: 0.0,
            },
            ProductivityZone {
                zone_type: ProductivityZoneType::PolarOcean,
                area: 2.6e7,
                npp: 80.0,
                energy_capture: 0.0,
            },
        ];

        // Calculate energy capture for each zone
        self.update_zone_energy_capture();
    }

    /// Update zone energy capture
    fn update_zone_energy_capture(&mut self) {
        for zone in &mut self.productivity_zones {
            // Energy = NPP * area * conversion factor
            // NPP is gC/m²/year, convert to W
            let npp_w = zone.npp * 1e12 / (365.0 * 24.0 * 3600.0); // Convert
            zone.energy_capture = npp_w * zone.area * 1e6; // W for entire zone
        }
    }

    /// Main tick: compute energy flow
    pub fn tick(&mut self, _seasonal_factor: f64) {
        let pi = std::f64::consts::PI;

        // ===== Stage 1: Stellar Input =====
        // Solar constant at this orbital distance
        let distance_m = self.orbital_radius * AU_TO_METERS;
        let luminosity_w = self.star_luminosity * 3.828e26; // Solar L in W
        self.metrics.stellar_input = luminosity_w / (4.0 * pi * distance_m * distance_m);

        // ===== Stage 2: Albedo Loss =====
        self.metrics.albedo_loss = self.metrics.stellar_input * self.planet_albedo;

        // ===== Stage 3: Atmospheric Absorption =====
        let absorbed = (self.metrics.stellar_input - self.metrics.albedo_loss)
            * (1.0 - self.atmosphere_transmittance);
        self.metrics.atmospheric_absorption = absorbed;

        // ===== Stage 4: Surface Irradiance =====
        self.metrics.surface_irradiance = self.metrics.stellar_input
            - self.metrics.albedo_loss
            - self.metrics.atmospheric_absorption;

        // ===== Stage 5: Photosynthetically Active Radiation =====
        // PAR is roughly 45% of total surface irradiance
        self.metrics.par = self.metrics.surface_irradiance * 0.45;

        // ===== Stage 6: Photosynthetic Capture =====
        self.metrics.photosynthetic_capture =
            self.metrics.par * self.photosynthetic_efficiency * self.biosphere_present;

        // ===== Stage 7: Net Primary Productivity =====
        // NPP is typically ~24% of photosynthetic capture
        self.metrics.npp = self.metrics.photosynthetic_capture * 0.24;

        // ===== Stage 8: Secondary Productivity =====
        // Energy transfers through food web with trophic efficiency
        let trophic_levels = AVG_TROPHIC_LEVELS;
        let food_web_efficiency = TROPHIC_EFFICIENCY.powf(trophic_levels - 1.0);
        self.metrics.secondary_productivity = self.metrics.npp * food_web_efficiency;

        // ===== Stage 9: Entity Available Energy =====
        // What's available to entities (including humans)
        self.metrics.entity_available = self.metrics.secondary_productivity * 0.1;

        // ===== Stage 10: Geothermal =====
        self.metrics.geothermal = GEOTHERMAL_FLOW;

        // ===== Total =====
        self.metrics.total_input = self.metrics.surface_irradiance + self.metrics.geothermal;

        // ===== Geographic Distribution =====
        self.update_geographic_distribution();

        // ===== Seasonal Variation =====
        self.update_seasonal_variation();

        // ===== Update Zones =====
        self.update_zone_energy_capture();
    }

    /// Update geographic distribution
    fn update_geographic_distribution(&mut self) {
        // Simplified: more energy at equator, less at poles
        // Based on latitude distribution
        let base = self.metrics.surface_irradiance;

        // Equatorial (0-30°): ~1.4x average
        self.geographic.equatorial = base * 1.4;

        // Temperate (30-60°): ~0.9x average
        self.geographic.temperate = base * 0.9;

        // Polar (60-90°): ~0.4x average
        self.geographic.polar = base * 0.4;
    }

    /// Update seasonal variation
    fn update_seasonal_variation(&mut self) {
        // Simplified seasonal variation
        let base = self.metrics.stellar_input;

        self.seasonal.summer_excess = base * 0.3;
        self.seasonal.winter_deficit = -base * 0.3;
        self.seasonal.annual_throughput = base * 365.0 * 24.0 * 3600.0;
        self.seasonal.seasonal_amplitude = 0.3;
    }

    /// Get habitability score based on energy availability
    pub fn habitability_score(&self) -> f64 {
        // Score based on energy availability relative to Earth
        let earth_entity_available = 0.2; // W/m² on Earth (very rough)
        let ratio = self.metrics.entity_available / earth_entity_available;

        // Score from 0 (no energy) to 1 (Earth-like or better)
        if ratio < 0.1 {
            ratio * 10.0
        } else {
            (ratio / 1.0).min(1.0)
        }
    }

    /// Get total biological productivity (Watts)
    pub fn total_productivity(&self) -> f64 {
        self.productivity_zones
            .iter()
            .map(|z| z.energy_capture)
            .sum()
    }

    /// Get summary string
    pub fn summary(&self) -> String {
        format!(
            "Energy Flow:\n\
            ├─ Stellar: {:.1} W/m²\n\
            ├─ Surface: {:.1} W/m²\n\
            ├─ Photosynthetic: {:.4} W/m²\n\
            ├─ NPP: {:.4} W/m²\n\
            ├─ Entity Available: {:.6} W/m²\n\
            └─ Total Productivity: {:.2e} W",
            self.metrics.stellar_input,
            self.metrics.surface_irradiance,
            self.metrics.photosynthetic_capture,
            self.metrics.npp,
            self.metrics.entity_available,
            self.total_productivity()
        )
    }
}

// ============================================================================
// Integration with Star
// ============================================================================

impl EnergyFlowSystem {
    /// Compute energy flow for a star and orbital distance
    pub fn from_star(star: &Star, orbital_radius: f64, albedo: f64) -> Self {
        let mut system = Self::new();

        // Get star luminosity
        let luminosity = star.luminosity;

        // Initialize with parameters
        system.initialize(luminosity, orbital_radius, albedo, 0.77);

        // Initial tick
        system.tick(1.0);

        system
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_flow_creation() {
        let efs = EnergyFlowSystem::new();
        assert!(efs.metrics.stellar_input >= 0.0);
    }

    #[test]
    fn test_habitability_score() {
        let mut efs = EnergyFlowSystem::new();
        efs.metrics.entity_available = 0.2;
        assert!(efs.habitability_score() > 0.5);
    }

    #[test]
    fn test_energy_summary() {
        let efs = EnergyFlowSystem::new();
        let summary = efs.summary();
        assert!(summary.contains("Energy Flow"));
    }
}
