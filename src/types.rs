//! Common types and type aliases for the Holonic Realms simulation

pub type Float = f64;

/// Octant - a division of the archetypical mind
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Octant(pub u8);

impl Octant {
    pub const O1: Octant = Octant(1);
    pub const O2: Octant = Octant(2);
    pub const O3: Octant = Octant(3);
    pub const O4: Octant = Octant(4);
    pub const O5: Octant = Octant(5);
    pub const O6: Octant = Octant(6);
    pub const O7: Octant = Octant(7);
    pub const O8: Octant = Octant(8);

    pub fn value(&self) -> u8 {
        self.0
    }
}

impl From<u8> for Octant {
    fn from(value: u8) -> Self {
        Octant(value)
    }
}

impl From<Octant> for u8 {
    fn from(octant: Octant) -> Self {
        octant.0
    }
}

/// Rung - a level in the octave
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rung(pub u8);

impl Rung {
    pub const R1: Rung = Rung(1);
    pub const R2: Rung = Rung(2);
    pub const R3: Rung = Rung(3);
    pub const R4: Rung = Rung(4);
    pub const R5: Rung = Rung(5);
    pub const R6: Rung = Rung(6);
    pub const R7: Rung = Rung(7);

    pub fn value(&self) -> u8 {
        self.0
    }
}

impl From<u8> for Rung {
    fn from(value: u8) -> Self {
        Rung(value)
    }
}

impl From<Rung> for u8 {
    fn from(rung: Rung) -> Self {
        rung.0
    }
}

/// Density - the density level of an entity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Density {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
}

impl Density {
    pub fn as_u8(&self) -> u8 {
        match self {
            Density::First => 1,
            Density::Second => 2,
            Density::Third => 3,
            Density::Fourth => 4,
            Density::Fifth => 5,
            Density::Sixth => 6,
            Density::Seventh => 7,
            Density::Eighth => 8,
        }
    }

    pub fn as_usize(&self) -> usize {
        self.as_u8() as usize
    }

    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => Density::First,
            2 => Density::Second,
            3 => Density::Third,
            4 => Density::Fourth,
            5 => Density::Fifth,
            6 => Density::Sixth,
            7 => Density::Seventh,
            8 => Density::Eighth,
            _ => Density::First,
        }
    }
}

impl std::fmt::Display for Density {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Density::First => write!(f, "1st Density"),
            Density::Second => write!(f, "2nd Density"),
            Density::Third => write!(f, "3rd Density"),
            Density::Fourth => write!(f, "4th Density"),
            Density::Fifth => write!(f, "5th Density"),
            Density::Sixth => write!(f, "6th Density"),
            Density::Seventh => write!(f, "7th Density"),
            Density::Eighth => write!(f, "8th Density"),
        }
    }
}

/// Holon ID - unique identifier for a holon
pub type HolonID = u64;

/// Environment ID - unique identifier for an environment
pub type EnvironmentID = u64;

/// Complex type - type of complex (Mind/Body/Spirit)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplexType {
    Mind,
    Body,
    Spirit,
    Choice,
    Complex,
    Experience,
    LesserCycleState,
}

/// Re-export ComplexType as `complex` for backward compatibility
pub use ComplexType as complex;

/// Polarity - STO/STS polarity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Polarity {
    ServiceToOthers,
    STO, // Alias for ServiceToOthers
    ServiceToSelf,
    STS, // Alias for ServiceToSelf
    Neutral,
    SinkholeOfIndifference,
}

impl std::fmt::Display for Polarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Polarity::Neutral => write!(f, "Neutral"),
            Polarity::STO => write!(f, "ServiceToOthers"),
            Polarity::STS => write!(f, "ServiceToSelf"),
            Polarity::ServiceToOthers => write!(f, "ServiceToOthers"),
            Polarity::ServiceToSelf => write!(f, "ServiceToSelf"),
            Polarity::SinkholeOfIndifference => write!(f, "SinkholeOfIndifference"),
        }
    }
}

/// Holonic level - scale of holonic organization

/// Holonic level - scale of holonic organization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HolonicLevel {
    Micro,
    Meso,
    Macro,
    Meta,
}

/// Health status - health state of an archetype or entity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Balanced,
    Warning,
    Pathological,
    PathologicalLow,
    PathologicalHigh,
    Imbalanced,
    Degraded,
}

impl std::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HealthStatus::Healthy => write!(f, "Healthy"),
            HealthStatus::Balanced => write!(f, "Balanced"),
            HealthStatus::Warning => write!(f, "Warning"),
            HealthStatus::Pathological => write!(f, "Pathological"),
            HealthStatus::PathologicalLow => write!(f, "Pathological Low"),
            HealthStatus::PathologicalHigh => write!(f, "Pathological High"),
            HealthStatus::Imbalanced => write!(f, "Imbalanced"),
            HealthStatus::Degraded => write!(f, "Degraded"),
        }
    }
}
