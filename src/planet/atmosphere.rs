//! Dynamic Atmosphere & Weather
//!
//! This module implements the dynamic atmosphere including composition,
//! pressure, temperature, wind, clouds, and weather systems.
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The atmosphere is the breath of the planet, mediating between
//! space and surface."

use rand::Rng;
use std::collections::HashMap;

// Re-export Season from cosmos for convenience
pub use crate::cosmos::planetary_formation::Season;

// ============================================================================
// Constants
// ============================================================================

/// Standard sea level pressure (Pa)
const SEA_LEVEL_PRESSURE: f64 = 101325.0;

/// Scale height of atmosphere (m)
const SCALE_HEIGHT: f64 = 8500.0;

/// Seconds per hour
const SECONDS_PER_HOUR: f64 = 3600.0;

/// Coriolis parameter (simplified)
const CORIOLIS_PARAMETER: f64 = 1e-4;

// ============================================================================
// Data Structures
// ============================================================================

/// Atmospheric composition
#[derive(Debug, Clone)]
pub struct AtmosphericComposition {
    /// Nitrogen fraction (Earth: 0.7808)
    pub nitrogen: f64,
    /// Oxygen fraction (Earth: 0.2095)
    pub oxygen: f64,
    /// Argon fraction (Earth: 0.0093)
    pub argon: f64,
    /// CO2 fraction (Earth: 0.0004)
    pub co2: f64,
    /// Water vapor fraction (variable, 0.0-0.04)
    pub water_vapor: f64,
    /// Methane fraction
    pub methane: f64,
    /// Neon, helium, etc.
    pub trace_gases: f64,
    /// Greenhouse factor (computed)
    pub greenhouse_effect: f64,
}

impl Default for AtmosphericComposition {
    fn default() -> Self {
        Self::earth_like()
    }
}

impl AtmosphericComposition {
    /// Create Earth-like atmosphere
    pub fn earth_like() -> Self {
        Self {
            nitrogen: 0.7808,
            oxygen: 0.2095,
            argon: 0.0093,
            co2: 0.0004,
            water_vapor: 0.01,
            methane: 0.000002,
            trace_gases: 0.000098,
            greenhouse_effect: 1.0,
        }
    }

    /// Compute greenhouse effect from composition
    pub fn compute_greenhouse(&mut self) {
        // Simplified greenhouse calculation
        self.greenhouse_effect =
            self.co2 * 100.0 + self.methane * 25000.0 + self.water_vapor * 50.0;
    }

    /// Chemistry tick - balance with biosphere
    pub fn chemistry_tick(&mut self, dt: f64, photosynthesis_rate: f64) {
        // Photosynthesis: CO2 → O2
        let photosynthesis = photosynthesis_rate * self.co2 * dt;
        self.co2 -= photosynthesis;
        self.oxygen += photosynthesis;

        // Respiration: O2 → CO2
        let respiration = self.oxygen * 0.0001 * dt;
        self.oxygen -= respiration;
        self.co2 += respiration;

        // Normalize
        self.normalize();
    }

    /// Normalize composition to sum to 1.0
    fn normalize(&mut self) {
        let total = self.nitrogen
            + self.oxygen
            + self.argon
            + self.co2
            + self.water_vapor
            + self.methane
            + self.trace_gases;

        if total > 0.0 && (total - 1.0).abs() > 0.001 {
            let factor = 1.0 / total;
            self.nitrogen *= factor;
            self.oxygen *= factor;
            self.argon *= factor;
            self.co2 *= factor;
            self.water_vapor *= factor;
            self.methane *= factor;
            self.trace_gases *= factor;
        }
    }
}

/// Pressure field cell
#[derive(Debug, Clone)]
pub struct PressureCell {
    pub position: (f64, f64),  // lat, lon
    pub pressure: f64,         // Pa
    pub pressure_anomaly: f64, // deviation from normal
}

/// Temperature field cell
#[derive(Debug, Clone)]
pub struct TemperatureCell {
    pub position: (f64, f64),
    pub temperature: f64, // Kelvin
    pub anomaly: f64,     // deviation from normal
}

/// Wind vector
#[derive(Debug, Clone, Default)]
pub struct WindVector {
    pub u: f64, // eastward (m/s)
    pub v: f64, // northward (m/s)
}

impl WindVector {
    pub fn magnitude(&self) -> f64 {
        (self.u * self.u + self.v * self.v).sqrt()
    }
}

/// Wind field cell
#[derive(Debug, Clone)]
pub struct WindCell {
    pub position: (f64, f64),
    pub wind: WindVector,
    pub gust_factor: f64,
}

/// Humidity field cell
#[derive(Debug, Clone)]
pub struct HumidityCell {
    pub position: (f64, f64),
    pub relative_humidity: f64, // 0.0-1.0
    pub dew_point: f64,         // K
}

/// Cloud formation
#[derive(Debug, Clone)]
pub struct CloudFormation {
    pub center: (f64, f64),
    pub radius: f64, // km
    pub cloud_type: CloudType,
    pub altitude: f64,      // km
    pub optical_depth: f64, // 0-1
    pub water_content: f64, // kg/m³
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloudType {
    Cirrus,       // High, wispy
    Cumulus,      // Puffy, fair weather
    Stratus,      // Layer, gray
    Cumulonimbus, // Storm, tall
    Nimbus,       // Rain-bearing
}

/// Weather front
#[derive(Debug, Clone)]
pub struct WeatherFront {
    pub front_type: FrontType,
    pub position: (f64, f64),
    pub direction: (f64, f64),   // movement vector
    pub speed: f64,              // km/h
    pub temperature_change: f64, // K
    pub precipitation_rate: f64, // mm/h
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrontType {
    Cold,
    Warm,
    Occluded,
    Stationary,
}

/// A storm system
#[derive(Debug, Clone)]
pub struct Storm {
    pub storm_type: StormType,
    pub center: (f64, f64),
    pub radius: f64,             // km
    pub intensity: f64,          // 0.0-1.0
    pub wind_speed: f64,         // m/s
    pub precipitation_rate: f64, // mm/h
    pub direction: (f64, f64),
    pub lifespan_remaining: f64, // hours
    pub pressure_drop: f64,      // Pa below normal
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StormType {
    /// Local convective storm
    Thunderstorm,
    /// Tropical cyclone
    Hurricane,
    /// Violent mesocyclone
    Tornado,
    /// Winter storm
    Blizzard,
    /// Desert wind storm
    Sandstorm,
    /// Extratropical cyclone
    Extratropical,
}

impl Storm {
    /// Create a new storm
    pub fn new(storm_type: StormType, center: (f64, f64), intensity: f64) -> Self {
        let (radius, wind_speed, precip, lifespan) = match storm_type {
            StormType::Thunderstorm => (50.0, 20.0, 20.0, 6.0),
            StormType::Hurricane => (300.0, 50.0, 50.0, 168.0),
            StormType::Tornado => (2.0, 100.0, 10.0, 1.0),
            StormType::Blizzard => (500.0, 20.0, 30.0, 24.0),
            StormType::Sandstorm => (100.0, 30.0, 0.0, 12.0),
            StormType::Extratropical => (1000.0, 25.0, 15.0, 72.0),
        };

        Self {
            storm_type,
            center,
            radius,
            intensity,
            wind_speed: wind_speed * intensity,
            precipitation_rate: precip * intensity,
            direction: (
                rand::thread_rng().gen::<f64>() - 0.5,
                rand::thread_rng().gen::<f64>() - 0.5,
            ),
            lifespan_remaining: lifespan,
            pressure_drop: intensity * 5000.0,
        }
    }

    /// Tick the storm
    pub fn tick(&mut self, dt: f64) {
        let dt_hours = dt / SECONDS_PER_HOUR;

        // Move storm
        self.center.0 += self.direction.0 * self.speed() * dt_hours / 111.0;
        self.center.1 += self.direction.1 * self.speed() * dt_hours / 111.0;

        // Decay
        self.lifespan_remaining -= dt_hours;
        self.intensity *= 0.999;

        // Wind around center (cyclonic)
        if matches!(
            self.storm_type,
            StormType::Hurricane | StormType::Extratropical
        ) {
            self.wind_speed = 20.0 + 80.0 * self.intensity;
        }
    }

    fn speed(&self) -> f64 {
        (self.direction.0.powi(2) + self.direction.1.powi(2)).sqrt() * 20.0
    }
}

// ============================================================================
// Main Atmosphere Structure
// ============================================================================

/// Dynamic atmosphere with weather systems
#[derive(Debug, Clone)]
pub struct DynamicAtmosphere {
    /// Atmospheric composition
    pub composition: AtmosphericComposition,
    /// Pressure field
    pub pressure_field: Vec<PressureCell>,
    /// Temperature field
    pub temperature_field: Vec<TemperatureCell>,
    /// Wind field
    pub wind_field: Vec<WindCell>,
    /// Humidity field
    pub humidity_field: Vec<HumidityCell>,
    /// Cloud formations
    pub cloud_formations: Vec<CloudFormation>,
    /// Weather fronts
    pub weather_fronts: Vec<WeatherFront>,
    /// Active storms
    pub storms: Vec<Storm>,
    /// Surface wind average (m/s)
    pub average_wind_speed: f64,
    /// Global precipitation (mm/day)
    pub global_precipitation: f64,
}

impl Default for DynamicAtmosphere {
    fn default() -> Self {
        Self::new()
    }
}

impl DynamicAtmosphere {
    /// Create a new atmosphere
    pub fn new() -> Self {
        let mut composition = AtmosphericComposition::earth_like();
        composition.compute_greenhouse();

        // Create fields (5-degree resolution)
        let mut pressure_field = Vec::new();
        let mut temperature_field = Vec::new();
        let mut wind_field = Vec::new();
        let mut humidity_field = Vec::new();

        for lat in (-90..90).step_by(5) {
            for lon in (-180..180).step_by(5) {
                let lat_f = lat as f64;

                // Pressure: higher at low latitudes due to temperature
                let base_pressure = SEA_LEVEL_PRESSURE * (1.0 - 0.0001 * lat_f * lat_f);
                pressure_field.push(PressureCell {
                    position: (lat_f, lon as f64),
                    pressure: base_pressure,
                    pressure_anomaly: 0.0,
                });

                // Temperature: hotter at equator, colder at poles
                let base_temp = 288.0 - 0.0065 * lat_f.abs() * 111.0; // Lapse rate
                temperature_field.push(TemperatureCell {
                    position: (lat_f, lon as f64),
                    temperature: base_temp,
                    anomaly: 0.0,
                });

                // Wind: trade winds, westerlies
                let (u, v) = if lat_f.abs() < 30.0 {
                    // Trade winds
                    (-5.0, if lat_f > 0.0 { -2.0 } else { 2.0 })
                } else if lat_f.abs() < 60.0 {
                    // Westerlies
                    (10.0, if lat_f > 0.0 { 2.0 } else { -2.0 })
                } else {
                    // Polar easterlies
                    (-5.0, 0.0)
                };

                wind_field.push(WindCell {
                    position: (lat_f, lon as f64),
                    wind: WindVector { u, v },
                    gust_factor: 1.0 + rand::thread_rng().gen::<f64>() * 0.3,
                });

                // Humidity: higher over oceans
                let lon_val = lon as f64;
                let lat_abs = lat_f.abs();
                let is_ocean = lon_val > -180.0 && lon_val < 180.0 && lat_abs < 60.0;
                let base_humidity = if is_ocean { 0.7 } else { 0.4 };

                humidity_field.push(HumidityCell {
                    position: (lat_f, lon as f64),
                    relative_humidity: base_humidity + rand::thread_rng().gen::<f64>() * 0.2,
                    dew_point: 280.0,
                });
            }
        }

        Self {
            composition,
            pressure_field,
            temperature_field,
            wind_field,
            humidity_field,
            cloud_formations: Vec::new(),
            weather_fronts: Vec::new(),
            storms: Vec::new(),
            average_wind_speed: 5.0,
            global_precipitation: 2.5,
        }
    }

    /// Main tick: advance atmospheric processes
    pub fn tick(&mut self, solar_radiation: f64, solar_angle: f64, season: Season, dt: f64) {
        // 1. Solar heating
        self.apply_solar_heating(solar_radiation, solar_angle);

        // 2. Greenhouse effect
        self.apply_greenhouse_effect(solar_radiation);

        // 3. Pressure gradient from temperature
        self.update_pressure_gradient(dt);

        // 4. Wind from pressure gradients + Coriolis
        self.update_wind_field(dt);

        // 5. Humidity from evaporation
        self.update_humidity(solar_radiation, dt);

        // 6. Cloud formation
        self.update_cloud_formations();

        // 7. Storm genesis
        self.check_storm_formation();

        // 8. Weather front movement
        for front in &mut self.weather_fronts {
            front.advance(dt);
        }

        // 9. Storm evolution
        self.storms.retain_mut(|storm| {
            storm.tick(dt);
            storm.lifespan_remaining > 0.0
        });

        // 10. Atmospheric chemistry
        self.composition.chemistry_tick(dt, 0.001);

        // 11. Calculate global averages
        self.update_averages();
    }

    /// Apply solar heating
    fn apply_solar_heating(&mut self, solar_radiation: f64, solar_angle: f64) {
        // Intensity depends on solar angle
        let intensity = solar_angle.max(0.0);

        for cell in &mut self.temperature_field {
            let lat = cell.position.0;

            // Diurnal variation
            let day_factor = intensity;

            // Latitude variation (more heat at equator)
            let lat_factor = 1.0 - (lat / 90.0).abs();

            // Heating
            let heating = solar_radiation * 0.001 * day_factor * lat_factor;
            cell.temperature += heating;

            // Night cooling
            cell.temperature -= 0.01 * (1.0 - day_factor);
        }
    }

    /// Apply greenhouse warming
    fn apply_greenhouse_effect(&mut self, solar_radiation: f64) {
        let greenhouse_warming = solar_radiation * self.composition.greenhouse_effect * 0.05;

        for cell in &mut self.temperature_field {
            cell.temperature += greenhouse_warming;
        }
    }

    /// Update pressure gradient
    fn update_pressure_gradient(&mut self, _dt: f64) {
        // High pressure at poles (cold), low at equator (warm)
        // First collect temperatures to avoid borrow issues
        let temps: Vec<_> = self
            .temperature_field
            .iter()
            .map(|c| (c.position, c.temperature))
            .collect();

        for (i, cell) in self.pressure_field.iter_mut().enumerate() {
            if i < temps.len() {
                let base_pressure = SEA_LEVEL_PRESSURE * (temps[i].1 / 288.0).powf(5.0);
                cell.pressure = base_pressure;
            }
        }
    }

    /// Update wind from pressure gradients
    fn update_wind_field(&mut self, dt: f64) {
        let dt_hours = dt / SECONDS_PER_HOUR;

        // Geostrophic wind approximation
        for cell in &mut self.wind_field {
            // Simplified: wind from pressure gradient, deflected by Coriolis
            let _lat = cell.position.0;
            let _coriolis = CORIOLIS_PARAMETER * _lat.sin();

            // Add some turbulence
            cell.wind.u += (rand::thread_rng().gen::<f64>() - 0.5) * 0.5 * dt_hours;
            cell.wind.v += (rand::thread_rng().gen::<f64>() - 0.5) * 0.5 * dt_hours;

            // Damping
            cell.wind.u *= 0.999;
            cell.wind.v *= 0.999;

            // Gusts
            if rand::thread_rng().gen::<f64>() < 0.01 {
                cell.gust_factor = 1.5;
            } else {
                cell.gust_factor = (cell.gust_factor * 0.99 + 1.0).min(1.5);
            }
        }
    }

    /// Update humidity
    fn update_humidity(&mut self, solar_radiation: f64, dt: f64) {
        // First collect temperatures to avoid borrow issues
        let temps: Vec<_> = self
            .temperature_field
            .iter()
            .map(|c| (c.position, c.temperature))
            .collect();

        for (i, cell) in self.humidity_field.iter_mut().enumerate() {
            let temp = if i < temps.len() { temps[i].1 } else { 288.0 };

            // Evaporation adds moisture
            let evaporation = solar_radiation * 0.00001 * dt;
            cell.relative_humidity = (cell.relative_humidity + evaporation).min(1.0);

            // Dew point from temperature
            cell.dew_point = temp - (1.0 - cell.relative_humidity) * 20.0;
        }
    }

    /// Update cloud formations
    fn update_cloud_formations(&mut self) {
        // Keep only significant clouds
        self.cloud_formations.retain(|c| c.water_content > 0.1);

        // Form new clouds where humidity is high
        for cell in &self.humidity_field {
            if cell.relative_humidity > 0.8 {
                if rand::thread_rng().gen::<f64>() < 0.001 {
                    self.cloud_formations.push(CloudFormation {
                        center: cell.position,
                        radius: 50.0 + rand::thread_rng().gen::<f64>() * 100.0,
                        cloud_type: CloudType::Cumulus,
                        altitude: 2.0 + rand::thread_rng().gen::<f64>() * 6.0,
                        optical_depth: 0.3 + rand::thread_rng().gen::<f64>() * 0.5,
                        water_content: cell.relative_humidity * 0.5,
                    });
                }
            }
        }
    }

    /// Check for storm formation
    fn check_storm_formation(&mut self) {
        // Storms form from warm moist air
        if rand::thread_rng().gen::<f64>() < 0.001 {
            let lat = (rand::thread_rng().gen::<f64>() - 0.5) * 120.0;
            let lon = (rand::thread_rng().gen::<f64>() - 0.5) * 360.0;

            let storm_type = if lat.abs() < 35.0 {
                // Tropical
                StormType::Hurricane
            } else {
                // Extratropical
                StormType::Extratropical
            };

            self.storms.push(Storm::new(storm_type, (lat, lon), 0.5));
        }

        // Thunderstorms in summer
        if rand::thread_rng().gen::<f64>() < 0.005 {
            let lat = (rand::thread_rng().gen::<f64>() - 0.5) * 60.0;
            let lon = (rand::thread_rng().gen::<f64>() - 0.5) * 360.0;
            self.storms
                .push(Storm::new(StormType::Thunderstorm, (lat, lon), 0.3));
        }
    }

    /// Update global averages
    fn update_averages(&mut self) {
        let total: f64 = self.wind_field.iter().map(|c| c.wind.magnitude()).sum();
        self.average_wind_speed = total / self.wind_field.len().max(1) as f64;

        self.global_precipitation = self.compute_global_precipitation();
    }

    /// Get global precipitation
    fn compute_global_precipitation(&self) -> f64 {
        let storm_precip: f64 = self.storms.iter().map(|s| s.precipitation_rate).sum();

        let front_precip: f64 = self
            .weather_fronts
            .iter()
            .map(|f| f.precipitation_rate)
            .sum();

        (storm_precip + front_precip) / 1000.0 // Normalize
    }

    /// Get temperature at a location
    pub fn temperature_at(&self, lat: f64, _lon: f64) -> f64 {
        for cell in &self.temperature_field {
            if (cell.position.0 - lat).abs() < 3.0 {
                return cell.temperature;
            }
        }
        288.0 // Default
    }

    /// Get pressure at altitude
    pub fn pressure_at_altitude(&self, altitude: f64) -> f64 {
        SEA_LEVEL_PRESSURE * (-altitude / SCALE_HEIGHT).exp()
    }

    /// Get wind at location
    pub fn wind_at(&self, lat: f64, lon: f64) -> WindVector {
        for cell in &self.wind_field {
            if (cell.position.0 - lat).abs() < 3.0 && (cell.position.1 - lon).abs() < 3.0 {
                return WindVector {
                    u: cell.wind.u * cell.gust_factor,
                    v: cell.wind.v * cell.gust_factor,
                };
            }
        }
        WindVector::default()
    }

    /// Get precipitation at location
    pub fn precipitation_at(&self, lat: f64, lon: f64) -> f64 {
        let mut precip = 0.0;

        // From storms
        for storm in &self.storms {
            let dist = ((storm.center.0 - lat).powi(2) + (storm.center.1 - lon).powi(2)).sqrt();
            if dist < storm.radius / 111.0 {
                precip += storm.precipitation_rate * (1.0 - dist / (storm.radius / 111.0));
            }
        }

        // From fronts
        for front in &self.weather_fronts {
            let dist = ((front.position.0 - lat).powi(2) + (front.position.1 - lon).powi(2)).sqrt();
            if dist < 5.0 {
                precip += front.precipitation_rate * (1.0 - dist / 5.0);
            }
        }

        precip
    }

    /// Get humidity at location
    pub fn humidity_at(&self, lat: f64, lon: f64) -> f64 {
        for cell in &self.humidity_field {
            if (cell.position.0 - lat).abs() < 3.0 && (cell.position.1 - lon).abs() < 3.0 {
                return cell.relative_humidity;
            }
        }
        0.5
    }

    /// Calculate atmospheric transmittance
    pub fn transmittance(&self, solar_angle: f64) -> f64 {
        if solar_angle <= 0.0 {
            return 0.0;
        }

        let air_mass = 1.0 / solar_angle.sin().max(0.01);

        // Simplified Beer-Lambert
        0.7_f64.powf(air_mass.powf(0.678))
    }
}

impl WeatherFront {
    /// Advance the front
    fn advance(&mut self, dt: f64) {
        let dt_hours = dt / SECONDS_PER_HOUR;

        self.position.0 += self.direction.0 * self.speed * dt_hours / 111.0;
        self.position.1 += self.direction.1 * self.speed * dt_hours / 111.0;
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atmosphere_creation() {
        let atm = DynamicAtmosphere::new();
        assert!(!atm.pressure_field.is_empty());
        assert!(!atm.temperature_field.is_empty());
    }

    #[test]
    fn test_transmittance() {
        let atm = DynamicAtmosphere::new();
        assert!(atm.transmittance(0.5) > 0.0);
        assert_eq!(atm.transmittance(0.0), 0.0);
    }

    #[test]
    fn test_storm_creation() {
        let storm = Storm::new(StormType::Hurricane, (20.0, -50.0), 0.8);
        assert_eq!(storm.storm_type, StormType::Hurricane);
    }
}
