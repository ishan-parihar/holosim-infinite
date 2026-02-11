//! # Larson Reciprocal Framework
//!
//! This module implements the Larson Reciprocal Framework for the Space/Time and Time/Space spectrum.
//!
//! ## Theoretical Foundation
//!
//! Space and time are not independent variables but **fundamentally reciprocal aspects of a unified progression** expressed through motion.
//!
//! ### First Fundamental Postulate
//!
//! The physical universe is composed entirely of one component, **motion**, existing in three dimensions, in discrete units, with two reciprocal aspects, space and time.
//!
//! ### Reciprocal Formulas
//!
//! - **v = s/t**: Motion in space (3D space, 1D time) - Many-ness dominant
//! - **v = t/s**: Motion in time (1D space, 3D time) - Oneness dominant
//! - **v = 1**: The Veil - transition point where qualitative break occurs
//!
//! ### Revolutionary Insight
//!
//! Both space and time have the SAME dimensions (both 3D). The difference is in how these dimensions manifest:
//! - In space/time: Space has 3D direction (freely navigable), time is scalar (1D linear flow)
//! - In time/space: Time has 3D direction (past/present/future accessible), space is scalar (fixed locus)
//!
//! This means space and time are complementary aspects, not separate realms - they are two expressions of the same underlying motion.

/// Represents the dimensions of space and time
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dimensionality {
    /// One-dimensional (scalar)
    OneDimensional,
    /// Three-dimensional (directional)
    ThreeDimensional,
}

impl Dimensionality {
    /// Returns the numeric value for calculations
    pub fn value(&self) -> f64 {
        match self {
            Dimensionality::OneDimensional => 1.0,
            Dimensionality::ThreeDimensional => 3.0,
        }
    }
}

/// Represents the spectrum position (space/time side or time/space side)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpectrumSide {
    /// Space/Time side (Many-ness dominant)
    SpaceTime,
    /// Time/Space side (Oneness dominant)
    TimeSpace,
}

/// Represents the spectrum quality (Oneness vs Many-ness)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpectrumQuality {
    /// Oneness dominant (time/space side)
    Oneness,
    /// Many-ness dominant (space/time side)
    ManyNess,
    /// The Veil (transitional state)
    Veil,
}

/// Represents a spectrum ratio (space/time or time/space)
#[derive(Debug, Clone)]
pub struct SpectrumRatio {
    /// Space component (1D or 3D)
    pub space_component: f64,
    /// Time component (1D or 3D)
    pub time_component: f64,
    /// Which side of the spectrum
    pub side: SpectrumSide,
}

impl SpectrumRatio {
    /// Creates a new spectrum ratio with specified side (legacy API for backward compatibility)
    pub fn new(ratio: f64, side: SpectrumSide) -> Self {
        match side {
            SpectrumSide::SpaceTime => SpectrumRatio {
                space_component: ratio,
                time_component: 1.0,
                side,
            },
            SpectrumSide::TimeSpace => SpectrumRatio {
                space_component: 1.0,
                time_component: ratio,
                side,
            },
        }
    }

    /// Creates a new Space/Time spectrum ratio (3D space, 1D time)
    pub fn space_time(space_component: f64, time_component: f64) -> Self {
        SpectrumRatio {
            space_component,
            time_component,
            side: SpectrumSide::SpaceTime,
        }
    }

    /// Creates a new Time/Space spectrum ratio (1D space, 3D time)
    pub fn time_space(space_component: f64, time_component: f64) -> Self {
        SpectrumRatio {
            space_component,
            time_component,
            side: SpectrumSide::TimeSpace,
        }
    }

    /// Calculates the velocity ratio
    /// - For space/time: v = s/t
    /// - For time/space: v = t/s
    pub fn calculate_ratio(&self) -> f64 {
        match self.side {
            SpectrumSide::SpaceTime => self.space_component / self.time_component,
            SpectrumSide::TimeSpace => self.time_component / self.space_component,
        }
    }

    /// Checks if this is at the Veil transition point (v = 1)
    pub fn is_veil_transition(&self) -> bool {
        let ratio = self.calculate_ratio();
        (ratio - 1.0).abs() < 0.01
    }

    /// Gets the spectrum quality
    pub fn quality(&self) -> SpectrumQuality {
        if self.is_veil_transition() {
            SpectrumQuality::Veil
        } else if self.side == SpectrumSide::SpaceTime {
            SpectrumQuality::ManyNess
        } else {
            SpectrumQuality::Oneness
        }
    }

    /// Gets the space dimensionality
    pub fn space_dimensionality(&self) -> Dimensionality {
        match self.side {
            SpectrumSide::SpaceTime => Dimensionality::ThreeDimensional,
            SpectrumSide::TimeSpace => Dimensionality::OneDimensional,
        }
    }

    /// Gets the time dimensionality
    pub fn time_dimensionality(&self) -> Dimensionality {
        match self.side {
            SpectrumSide::SpaceTime => Dimensionality::OneDimensional,
            SpectrumSide::TimeSpace => Dimensionality::ThreeDimensional,
        }
    }
}

/// Represents a scalar motion unit
#[derive(Debug, Clone)]
pub struct ScalarMotionUnit {
    /// The spectrum ratio for this motion unit
    pub ratio: SpectrumRatio,
    /// The magnitude of motion
    pub magnitude: f64,
    /// The phase angle (for resonant standing waves)
    pub phase: f64,
}

impl ScalarMotionUnit {
    /// Creates a new scalar motion unit
    pub fn new(ratio: SpectrumRatio, magnitude: f64) -> Self {
        ScalarMotionUnit {
            ratio,
            magnitude,
            phase: 0.0,
        }
    }

    /// Creates a scalar motion unit with phase
    pub fn with_phase(ratio: SpectrumRatio, magnitude: f64, phase: f64) -> Self {
        ScalarMotionUnit {
            ratio,
            magnitude,
            phase,
        }
    }

    /// Calculates the velocity
    pub fn velocity(&self) -> f64 {
        self.ratio.calculate_ratio() * self.magnitude
    }

    /// Checks if this motion unit is at the Veil
    pub fn at_veil(&self) -> bool {
        self.ratio.is_veil_transition()
    }
}

/// Represents a resonant standing wave
#[derive(Debug, Clone)]
pub struct ResonantStandingWave {
    /// The scalar motion units that compose this wave
    pub motion_units: Vec<ScalarMotionUnit>,
    /// The fundamental frequency
    pub fundamental_frequency: f64,
    /// The harmonic number
    pub harmonic: usize,
}

impl ResonantStandingWave {
    /// Creates a new resonant standing wave
    pub fn new(
        motion_units: Vec<ScalarMotionUnit>,
        fundamental_frequency: f64,
        harmonic: usize,
    ) -> Self {
        ResonantStandingWave {
            motion_units,
            fundamental_frequency,
            harmonic,
        }
    }

    /// Calculates the resonant frequency
    pub fn resonant_frequency(&self) -> f64 {
        self.fundamental_frequency * self.harmonic as f64
    }

    /// Checks if the wave is stable (all motion units are in phase)
    pub fn is_stable(&self) -> bool {
        if self.motion_units.is_empty() {
            return false;
        }

        let first_phase = self.motion_units[0].phase;
        self.motion_units
            .iter()
            .all(|unit| (unit.phase - first_phase).abs() < 0.1)
    }

    /// Gets the dominant spectrum quality
    pub fn dominant_quality(&self) -> SpectrumQuality {
        let veil_count = self.motion_units.iter().filter(|u| u.at_veil()).count();
        let oneness_count = self
            .motion_units
            .iter()
            .filter(|u| u.ratio.quality() == SpectrumQuality::Oneness)
            .count();
        let manyness_count = self
            .motion_units
            .iter()
            .filter(|u| u.ratio.quality() == SpectrumQuality::ManyNess)
            .count();

        if veil_count > 0 {
            SpectrumQuality::Veil
        } else if oneness_count >= manyness_count {
            SpectrumQuality::Oneness
        } else {
            SpectrumQuality::ManyNess
        }
    }
}

/// Represents a dimensional structure emerging from standing waves
#[derive(Debug, Clone)]
pub struct DimensionalStructure {
    /// The resonant standing waves that compose this structure
    pub standing_waves: Vec<ResonantStandingWave>,
    /// The dimension count (1D, 2D, 3D, etc.)
    pub dimensions: usize,
    /// The stability factor (0.0 to 1.0)
    pub stability: f64,
}

impl DimensionalStructure {
    /// Creates a new dimensional structure
    pub fn new(standing_waves: Vec<ResonantStandingWave>, dimensions: usize) -> Self {
        // Calculate stability based on wave coherence
        let stability = if standing_waves.is_empty() {
            0.0
        } else {
            let stable_count = standing_waves.iter().filter(|w| w.is_stable()).count();
            stable_count as f64 / standing_waves.len() as f64
        };

        DimensionalStructure {
            standing_waves,
            dimensions,
            stability,
        }
    }

    /// Checks if this structure is stable
    pub fn is_stable(&self) -> bool {
        self.stability > 0.8
    }

    /// Gets the dominant spectrum quality
    pub fn dominant_quality(&self) -> SpectrumQuality {
        if self.standing_waves.is_empty() {
            return SpectrumQuality::Veil;
        }

        let veil_count = self
            .standing_waves
            .iter()
            .filter(|w| w.dominant_quality() == SpectrumQuality::Veil)
            .count();
        let oneness_count = self
            .standing_waves
            .iter()
            .filter(|w| w.dominant_quality() == SpectrumQuality::Oneness)
            .count();
        let manyness_count = self
            .standing_waves
            .iter()
            .filter(|w| w.dominant_quality() == SpectrumQuality::ManyNess)
            .count();

        if veil_count > 0 {
            SpectrumQuality::Veil
        } else if oneness_count >= manyness_count {
            SpectrumQuality::Oneness
        } else {
            SpectrumQuality::ManyNess
        }
    }
}

/// The Larson Reciprocal Framework
#[derive(Debug, Clone)]
pub struct LarsonFramework {
    /// The scalar motion units in the framework
    pub motion_units: Vec<ScalarMotionUnit>,
    /// The resonant standing waves
    pub standing_waves: Vec<ResonantStandingWave>,
    /// The dimensional structures
    pub dimensional_structures: Vec<DimensionalStructure>,
}

impl LarsonFramework {
    /// Creates a new Larson framework
    pub fn new() -> Self {
        LarsonFramework {
            motion_units: Vec::new(),
            standing_waves: Vec::new(),
            dimensional_structures: Vec::new(),
        }
    }

    /// Adds a scalar motion unit
    pub fn add_motion_unit(&mut self, unit: ScalarMotionUnit) {
        self.motion_units.push(unit);
    }

    /// Organizes motion units into resonant standing waves
    pub fn organize_into_waves(&mut self, fundamental_frequency: f64) {
        // Group motion units by similar ratios and phases
        let mut waves: Vec<Vec<ScalarMotionUnit>> = Vec::new();

        for unit in &self.motion_units {
            let mut found_wave = false;

            for wave in &mut waves {
                // Check if this unit can join the wave (similar ratio and phase)
                if let Some(first) = wave.first() {
                    let ratio_diff =
                        (unit.ratio.calculate_ratio() - first.ratio.calculate_ratio()).abs();
                    let phase_diff = (unit.phase - first.phase).abs();

                    if ratio_diff < 0.5 && phase_diff < 0.5 {
                        wave.push(unit.clone());
                        found_wave = true;
                        break;
                    }
                }
            }

            if !found_wave {
                waves.push(vec![unit.clone()]);
            }
        }

        // Convert wave groups into resonant standing waves
        for (harmonic, wave_units) in waves.iter().enumerate() {
            if wave_units.len() > 1 {
                let wave = ResonantStandingWave::new(
                    wave_units.clone(),
                    fundamental_frequency,
                    harmonic + 1,
                );
                self.standing_waves.push(wave);
            }
        }
    }

    /// Organizes standing waves into dimensional structures
    pub fn organize_into_dimensions(&mut self) {
        // Group standing waves by dominant quality
        let mut oneness_waves: Vec<ResonantStandingWave> = Vec::new();
        let mut manyness_waves: Vec<ResonantStandingWave> = Vec::new();

        for wave in &self.standing_waves {
            match wave.dominant_quality() {
                SpectrumQuality::Oneness => oneness_waves.push(wave.clone()),
                SpectrumQuality::ManyNess => manyness_waves.push(wave.clone()),
                _ => {}
            }
        }

        // Create dimensional structures from stable waves
        if !oneness_waves.is_empty() {
            let structure = DimensionalStructure::new(oneness_waves, 3);
            if structure.is_stable() {
                self.dimensional_structures.push(structure);
            }
        }

        if !manyness_waves.is_empty() {
            let structure = DimensionalStructure::new(manyness_waves.clone(), 3);
            if structure.is_stable() {
                self.dimensional_structures.push(structure);
            }
        }
    }

    /// Gets the spectrum continuum position
    pub fn spectrum_continuum(&self) -> Vec<SpectrumRatio> {
        self.motion_units.iter().map(|u| u.ratio.clone()).collect()
    }

    /// Checks if the framework has dimensional structures
    pub fn has_dimensional_structures(&self) -> bool {
        !self.dimensional_structures.is_empty()
    }
}

impl Default for LarsonFramework {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_spectrum_ratio_space_time() {
        let ratio = SpectrumRatio::space_time(3.0, 1.0);
        assert_eq!(ratio.side, SpectrumSide::SpaceTime);
        assert_eq!(ratio.space_component, 3.0);
        assert_eq!(ratio.time_component, 1.0);
        assert_eq!(ratio.calculate_ratio(), 3.0);
        assert_eq!(ratio.quality(), SpectrumQuality::ManyNess);
    }

    #[test]
    fn test_spectrum_ratio_time_space() {
        let ratio = SpectrumRatio::time_space(1.0, 3.0);
        assert_eq!(ratio.side, SpectrumSide::TimeSpace);
        assert_eq!(ratio.space_component, 1.0);
        assert_eq!(ratio.time_component, 3.0);
        assert_eq!(ratio.calculate_ratio(), 3.0);
        assert_eq!(ratio.quality(), SpectrumQuality::Oneness);
    }

    #[test]
    fn test_spectrum_ratio_veil_transition() {
        let ratio = SpectrumRatio::space_time(1.0, 1.0);
        assert!(ratio.is_veil_transition());
        assert_eq!(ratio.quality(), SpectrumQuality::Veil);
    }

    #[test]
    fn test_dimensionality_values() {
        assert_eq!(Dimensionality::OneDimensional.value(), 1.0);
        assert_eq!(Dimensionality::ThreeDimensional.value(), 3.0);
    }

    #[test]
    fn test_scalar_motion_unit() {
        let ratio = SpectrumRatio::space_time(3.0, 1.0);
        let unit = ScalarMotionUnit::new(ratio.clone(), 2.0);

        assert_eq!(unit.magnitude, 2.0);
        assert_eq!(unit.phase, 0.0);
        assert_eq!(unit.velocity(), 6.0);
        assert!(!unit.at_veil());
    }

    #[test]
    fn test_scalar_motion_unit_at_veil() {
        let ratio = SpectrumRatio::space_time(1.0, 1.0);
        let unit = ScalarMotionUnit::new(ratio, 1.0);
        assert!(unit.at_veil());
    }

    #[test]
    fn test_scalar_motion_unit_with_phase() {
        let ratio = SpectrumRatio::space_time(3.0, 1.0);
        let unit = ScalarMotionUnit::with_phase(ratio, 2.0, PI / 2.0);

        assert_eq!(unit.phase, PI / 2.0);
    }

    #[test]
    fn test_resonant_standing_wave() {
        let ratio1 = SpectrumRatio::space_time(3.0, 1.0);
        let ratio2 = SpectrumRatio::space_time(3.0, 1.0);

        let unit1 = ScalarMotionUnit::with_phase(ratio1, 2.0, 0.0);
        let unit2 = ScalarMotionUnit::with_phase(ratio2, 2.0, 0.05);

        let wave = ResonantStandingWave::new(vec![unit1, unit2], 1.0, 1);

        assert_eq!(wave.harmonic, 1);
        assert_eq!(wave.resonant_frequency(), 1.0);
        assert!(wave.is_stable());
        assert_eq!(wave.dominant_quality(), SpectrumQuality::ManyNess);
    }

    #[test]
    fn test_resonant_standing_wave_unstable() {
        let ratio1 = SpectrumRatio::space_time(3.0, 1.0);
        let ratio2 = SpectrumRatio::space_time(3.0, 1.0);

        let unit1 = ScalarMotionUnit::with_phase(ratio1, 2.0, 0.0);
        let unit2 = ScalarMotionUnit::with_phase(ratio2, 2.0, 1.0);

        let wave = ResonantStandingWave::new(vec![unit1, unit2], 1.0, 1);

        assert!(!wave.is_stable());
    }

    #[test]
    fn test_dimensional_structure() {
        let ratio = SpectrumRatio::space_time(3.0, 1.0);
        let unit = ScalarMotionUnit::new(ratio, 2.0);
        let wave = ResonantStandingWave::new(vec![unit], 1.0, 1);

        let structure = DimensionalStructure::new(vec![wave], 3);

        assert_eq!(structure.dimensions, 3);
        assert!(structure.is_stable());
        assert_eq!(structure.dominant_quality(), SpectrumQuality::ManyNess);
    }

    #[test]
    fn test_dimensional_structure_unstable() {
        let ratio1 = SpectrumRatio::space_time(3.0, 1.0);
        let ratio2 = SpectrumRatio::space_time(3.0, 1.0);

        let unit1 = ScalarMotionUnit::with_phase(ratio1, 2.0, 0.0);
        let unit2 = ScalarMotionUnit::with_phase(ratio2, 2.0, 1.0);

        let wave = ResonantStandingWave::new(vec![unit1, unit2], 1.0, 1);
        let structure = DimensionalStructure::new(vec![wave], 3);

        assert!(!structure.is_stable());
    }

    #[test]
    fn test_larson_framework() {
        let mut framework = LarsonFramework::new();

        let ratio = SpectrumRatio::space_time(3.0, 1.0);
        let unit = ScalarMotionUnit::new(ratio, 2.0);
        framework.add_motion_unit(unit);

        assert_eq!(framework.motion_units.len(), 1);
        assert!(!framework.has_dimensional_structures());
    }

    #[test]
    fn test_larson_framework_organize_waves() {
        let mut framework = LarsonFramework::new();

        let ratio = SpectrumRatio::space_time(3.0, 1.0);
        let unit1 = ScalarMotionUnit::with_phase(ratio.clone(), 2.0, 0.0);
        let unit2 = ScalarMotionUnit::with_phase(ratio, 2.0, 0.05);

        framework.add_motion_unit(unit1);
        framework.add_motion_unit(unit2);

        framework.organize_into_waves(1.0);

        assert_eq!(framework.standing_waves.len(), 1);
        assert!(framework.standing_waves[0].is_stable());
    }

    #[test]
    fn test_larson_framework_organize_dimensions() {
        let mut framework = LarsonFramework::new();

        let ratio = SpectrumRatio::space_time(3.0, 1.0);
        let unit1 = ScalarMotionUnit::with_phase(ratio.clone(), 2.0, 0.0);
        let unit2 = ScalarMotionUnit::with_phase(ratio, 2.0, 0.05);

        framework.add_motion_unit(unit1);
        framework.add_motion_unit(unit2);

        framework.organize_into_waves(1.0);
        framework.organize_into_dimensions();

        assert!(framework.has_dimensional_structures());
    }

    #[test]
    fn test_spectrum_continuum() {
        let mut framework = LarsonFramework::new();

        let ratio1 = SpectrumRatio::space_time(3.0, 1.0);
        let ratio2 = SpectrumRatio::time_space(1.0, 3.0);

        framework.add_motion_unit(ScalarMotionUnit::new(ratio1, 2.0));
        framework.add_motion_unit(ScalarMotionUnit::new(ratio2, 2.0));

        let continuum = framework.spectrum_continuum();

        assert_eq!(continuum.len(), 2);
        assert_eq!(continuum[0].quality(), SpectrumQuality::ManyNess);
        assert_eq!(continuum[1].quality(), SpectrumQuality::Oneness);
    }

    #[test]
    fn test_space_dimensionality() {
        let ratio = SpectrumRatio::space_time(3.0, 1.0);
        assert_eq!(
            ratio.space_dimensionality(),
            Dimensionality::ThreeDimensional
        );
        assert_eq!(ratio.time_dimensionality(), Dimensionality::OneDimensional);

        let ratio = SpectrumRatio::time_space(1.0, 3.0);
        assert_eq!(ratio.space_dimensionality(), Dimensionality::OneDimensional);
        assert_eq!(
            ratio.time_dimensionality(),
            Dimensionality::ThreeDimensional
        );
    }

    #[test]
    fn test_resonant_frequency() {
        let ratio = SpectrumRatio::space_time(3.0, 1.0);
        let unit = ScalarMotionUnit::new(ratio, 2.0);
        let wave = ResonantStandingWave::new(vec![unit], 2.0, 3);

        assert_eq!(wave.resonant_frequency(), 6.0);
    }
}
