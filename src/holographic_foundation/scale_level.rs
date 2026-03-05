//! Unified Scale Level for Holographic Simulation
//!
//! This is the CANONICAL scale level definition that aligns with:
//! - The Density Octave (1st through 8th density)
//! - The Space/Time ↔ Time/Space spectrum
//! - The 8-level multi-scale field representation
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "At each sub-level of the density octave, individual and collective emerge SIMULTANEOUSLY:
//!  - Atoms and galaxies form together (1st Density - Atomic Realm)
//!  - Molecules and planets form together (1st Density - Molecular Realm)
//!  - Cells and Gaia-System form together (2nd Density - Cellular Realm)
//!  - Eukaryotes and ecosystems form together (2nd Density - Complex Life Realm)
//!  - Neuronal-organisms and societies form together (3rd Density - Conscious Life Realm)"
//!
//! From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
//! "The holographic field is stored at 7 scale levels:
//!  - quantum: 10^-35 m (Level 0)
//!  - atomic: 10^-15 m (Level 1)
//!  - cellular: 10^-6 m (Level 2)
//!  - biological: 10^0 m (Level 3)
//!  - planetary: 10^7 m (Level 4)
//!  - stellar: 10^13 m (Level 5)
//!  - galactic: 10^21 m (Level 6)
//!  - cosmic: 10^26 m (Level 7)"

use std::fmt;

use crate::types::Float;

pub const NUM_SCALE_LEVELS: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(u8)]
pub enum ScaleLevel {
    Quantum = 0,
    Atomic = 1,
    Molecular = 2,
    Cellular = 3,
    #[default]
    Biological = 4,
    Planetary = 5,
    Stellar = 6,
    Cosmic = 7,
}

impl ScaleLevel {
    pub fn characteristic_length(&self) -> Float {
        match self {
            ScaleLevel::Quantum => 1.6e-35,
            ScaleLevel::Atomic => 1e-10,
            ScaleLevel::Molecular => 1e-9,
            ScaleLevel::Cellular => 1e-5,
            ScaleLevel::Biological => 1.0,
            ScaleLevel::Planetary => 6.371e6,
            ScaleLevel::Stellar => 1.496e11,
            ScaleLevel::Cosmic => 3.086e22,
        }
    }

    pub fn min_size(&self) -> Float {
        match self {
            ScaleLevel::Quantum => 1e-35,
            ScaleLevel::Atomic => 1e-12,
            ScaleLevel::Molecular => 1e-10,
            ScaleLevel::Cellular => 1e-6,
            ScaleLevel::Biological => 1e-2,
            ScaleLevel::Planetary => 1e6,
            ScaleLevel::Stellar => 1e10,
            ScaleLevel::Cosmic => 1e22,
        }
    }

    pub fn max_size(&self) -> Float {
        match self {
            ScaleLevel::Quantum => 1e-15,
            ScaleLevel::Atomic => 1e-10,
            ScaleLevel::Molecular => 1e-8,
            ScaleLevel::Cellular => 1e-3,
            ScaleLevel::Biological => 1e4,
            ScaleLevel::Planetary => 1e9,
            ScaleLevel::Stellar => 1e18,
            ScaleLevel::Cosmic => 1e26,
        }
    }

    pub fn finer(&self) -> Option<ScaleLevel> {
        match self {
            ScaleLevel::Quantum => None,
            ScaleLevel::Atomic => Some(ScaleLevel::Quantum),
            ScaleLevel::Molecular => Some(ScaleLevel::Atomic),
            ScaleLevel::Cellular => Some(ScaleLevel::Molecular),
            ScaleLevel::Biological => Some(ScaleLevel::Cellular),
            ScaleLevel::Planetary => Some(ScaleLevel::Biological),
            ScaleLevel::Stellar => Some(ScaleLevel::Planetary),
            ScaleLevel::Cosmic => Some(ScaleLevel::Stellar),
        }
    }

    pub fn coarser(&self) -> Option<ScaleLevel> {
        match self {
            ScaleLevel::Quantum => Some(ScaleLevel::Atomic),
            ScaleLevel::Atomic => Some(ScaleLevel::Molecular),
            ScaleLevel::Molecular => Some(ScaleLevel::Cellular),
            ScaleLevel::Cellular => Some(ScaleLevel::Biological),
            ScaleLevel::Biological => Some(ScaleLevel::Planetary),
            ScaleLevel::Planetary => Some(ScaleLevel::Stellar),
            ScaleLevel::Stellar => Some(ScaleLevel::Cosmic),
            ScaleLevel::Cosmic => None,
        }
    }

    pub fn from_density(density: u8) -> Option<ScaleLevel> {
        match density {
            1 => Some(ScaleLevel::Quantum),
            2 => Some(ScaleLevel::Atomic),
            3 => Some(ScaleLevel::Molecular),
            4 => Some(ScaleLevel::Cellular),
            5 => Some(ScaleLevel::Biological),
            6 => Some(ScaleLevel::Planetary),
            7 => Some(ScaleLevel::Stellar),
            8 => Some(ScaleLevel::Cosmic),
            _ => None,
        }
    }

    pub fn to_density(&self) -> u8 {
        match self {
            ScaleLevel::Quantum => 1,
            ScaleLevel::Atomic => 2,
            ScaleLevel::Molecular => 3,
            ScaleLevel::Cellular => 4,
            ScaleLevel::Biological => 5,
            ScaleLevel::Planetary => 6,
            ScaleLevel::Stellar => 7,
            ScaleLevel::Cosmic => 8,
        }
    }

    pub fn as_u8(&self) -> u8 {
        *self as u8
    }

    pub fn from_u8(index: u8) -> Option<ScaleLevel> {
        match index {
            0 => Some(ScaleLevel::Quantum),
            1 => Some(ScaleLevel::Atomic),
            2 => Some(ScaleLevel::Molecular),
            3 => Some(ScaleLevel::Cellular),
            4 => Some(ScaleLevel::Biological),
            5 => Some(ScaleLevel::Planetary),
            6 => Some(ScaleLevel::Stellar),
            7 => Some(ScaleLevel::Cosmic),
            _ => None,
        }
    }

    pub fn index(&self) -> usize {
        *self as usize
    }

    pub fn from_index(index: usize) -> Option<Self> {
        Self::from_u8(index as u8)
    }

    pub fn next(&self) -> Option<ScaleLevel> {
        self.coarser()
    }

    pub fn prev(&self) -> Option<ScaleLevel> {
        self.finer()
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            ScaleLevel::Quantum => "Quantum",
            ScaleLevel::Atomic => "Atomic",
            ScaleLevel::Molecular => "Molecular",
            ScaleLevel::Cellular => "Cellular",
            ScaleLevel::Biological => "Biological",
            ScaleLevel::Planetary => "Planetary",
            ScaleLevel::Stellar => "Stellar",
            ScaleLevel::Cosmic => "Cosmic",
        }
    }

    pub fn log_center(&self) -> Float {
        (self.min_size().log10() + self.max_size().log10()) / 2.0
    }

    pub fn physics_mode(&self) -> PhysicsMode {
        match self {
            ScaleLevel::Quantum => PhysicsMode::Quantum,
            ScaleLevel::Atomic => PhysicsMode::Quantum,
            ScaleLevel::Molecular => PhysicsMode::SpaceTime,
            ScaleLevel::Cellular => PhysicsMode::SpaceTime,
            ScaleLevel::Biological => PhysicsMode::SpaceTime,
            ScaleLevel::Planetary => PhysicsMode::SpaceTime,
            ScaleLevel::Stellar => PhysicsMode::SpaceTime,
            ScaleLevel::Cosmic => PhysicsMode::TimeSpace,
        }
    }

    pub fn collective_counterpart(&self) -> &'static str {
        match self {
            ScaleLevel::Quantum => "Quantum Fields",
            ScaleLevel::Atomic => "Galaxies",
            ScaleLevel::Molecular => "Planets",
            ScaleLevel::Cellular => "Gaia System",
            ScaleLevel::Biological => "Ecosystems",
            ScaleLevel::Planetary => "Solar Systems",
            ScaleLevel::Stellar => "Galactic Logoi",
            ScaleLevel::Cosmic => "Intelligent Infinity",
        }
    }

    pub fn individual_manifestation(&self) -> &'static str {
        match self {
            ScaleLevel::Quantum => "Quantum Particles",
            ScaleLevel::Atomic => "Atoms",
            ScaleLevel::Molecular => "Molecules",
            ScaleLevel::Cellular => "Cells",
            ScaleLevel::Biological => "Organisms",
            ScaleLevel::Planetary => "Civilizations",
            ScaleLevel::Stellar => "Solar Logoi",
            ScaleLevel::Cosmic => "Octave Completion",
        }
    }
}

impl fmt::Display for ScaleLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (10^{} m) - {} ↔ {}",
            self.display_name(),
            self.log_center() as i32,
            self.individual_manifestation(),
            self.collective_counterpart()
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PhysicsMode {
    SpaceTime,
    TimeSpace,
    Quantum,
}

impl PhysicsMode {
    pub fn display_name(&self) -> &'static str {
        match self {
            PhysicsMode::SpaceTime => "Space/Time (Classical)",
            PhysicsMode::TimeSpace => "Time/Space (Metaphysical)",
            PhysicsMode::Quantum => "Quantum (Veil Transition)",
        }
    }
}

impl fmt::Display for PhysicsMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

pub fn all_scale_levels() -> [ScaleLevel; NUM_SCALE_LEVELS] {
    [
        ScaleLevel::Quantum,
        ScaleLevel::Atomic,
        ScaleLevel::Molecular,
        ScaleLevel::Cellular,
        ScaleLevel::Biological,
        ScaleLevel::Planetary,
        ScaleLevel::Stellar,
        ScaleLevel::Cosmic,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_density_mapping() {
        for density in 1..=8 {
            let scale = ScaleLevel::from_density(density).unwrap();
            assert_eq!(scale.to_density(), density);
        }
    }

    #[test]
    fn test_scale_navigation() {
        let mut current = ScaleLevel::Quantum;
        let mut count = 0;
        while let Some(next) = current.coarser() {
            current = next;
            count += 1;
        }
        assert_eq!(count, 7);
        assert_eq!(current, ScaleLevel::Cosmic);
    }

    #[test]
    fn test_characteristic_lengths() {
        assert!(
            ScaleLevel::Quantum.characteristic_length()
                < ScaleLevel::Atomic.characteristic_length()
        );
        assert!(
            ScaleLevel::Atomic.characteristic_length()
                < ScaleLevel::Molecular.characteristic_length()
        );
        assert!(
            ScaleLevel::Molecular.characteristic_length()
                < ScaleLevel::Cellular.characteristic_length()
        );
    }

    #[test]
    fn test_physics_modes() {
        assert_eq!(ScaleLevel::Quantum.physics_mode(), PhysicsMode::Quantum);
        assert_eq!(
            ScaleLevel::Biological.physics_mode(),
            PhysicsMode::SpaceTime
        );
        assert_eq!(ScaleLevel::Cosmic.physics_mode(), PhysicsMode::TimeSpace);
    }

    #[test]
    fn test_simultaneous_emergence() {
        assert_eq!(ScaleLevel::Atomic.collective_counterpart(), "Galaxies");
        assert_eq!(ScaleLevel::Molecular.collective_counterpart(), "Planets");
        assert_eq!(ScaleLevel::Cellular.collective_counterpart(), "Gaia System");
    }
}
