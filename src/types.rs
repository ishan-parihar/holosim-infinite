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

    /// Veil transparency based on density
    /// Higher density = more transparent veil
    /// Returns 0.0 to 1.0 where 1.0 is fully transparent
    pub fn veil_transparency(&self) -> Float {
        match self {
            Density::First => 0.0, // Fully veiled
            Density::Second => 0.1,
            Density::Third => 0.3,
            Density::Fourth => 0.5, // Half transparent
            Density::Fifth => 0.7,
            Density::Sixth => 0.9,
            Density::Seventh => 1.0, // Fully transparent
            Density::Eighth => 1.0,  // Beyond the spectrum
        }
    }

    /// Veil thickness by density
    /// Lower density = thicker veil
    /// Returns 0.0 to 1.0 where 0.0 is no veil
    pub fn veil_thickness(&self) -> Float {
        match self {
            Density::First => 0.95, // Almost completely opaque
            Density::Second => 0.85,
            Density::Third => 0.70, // 3rd density - thick veil
            Density::Fourth => 0.40,
            Density::Fifth => 0.20,
            Density::Sixth => 0.10,
            Density::Seventh => 0.0, // No veil
            Density::Eighth => 0.0,  // Beyond veil
        }
    }

    /// Direction of evolution on spectrum
    /// Higher density = faster evolution toward time-dominance
    /// Returns factor determining evolution speed
    pub fn evolution_direction(&self) -> Float {
        match self {
            Density::First => 0.1, // Slow evolution
            Density::Second => 0.2,
            Density::Third => 0.3,
            Density::Fourth => 0.5,
            Density::Fifth => 0.7,
            Density::Sixth => 1.0, // Fast evolution
            Density::Seventh => 1.5,
            Density::Eighth => 2.0,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_density_veil_transparency() {
        // From V5 Phase 1 specifications
        assert_eq!(
            Density::First.veil_transparency(),
            0.0,
            "1st density should be fully veiled"
        );
        assert_eq!(Density::Second.veil_transparency(), 0.1);
        assert_eq!(Density::Third.veil_transparency(), 0.3);
        assert_eq!(
            Density::Fourth.veil_transparency(),
            0.5,
            "4th density should be half transparent"
        );
        assert_eq!(Density::Fifth.veil_transparency(), 0.7);
        assert_eq!(Density::Sixth.veil_transparency(), 0.9);
        assert_eq!(
            Density::Seventh.veil_transparency(),
            1.0,
            "7th density should be fully transparent"
        );
        assert_eq!(
            Density::Eighth.veil_transparency(),
            1.0,
            "8th density should be fully transparent"
        );

        // Verify transparency increases with density
        assert!(Density::First.veil_transparency() < Density::Second.veil_transparency());
        assert!(Density::Third.veil_transparency() < Density::Fourth.veil_transparency());
        assert!(Density::Fifth.veil_transparency() < Density::Sixth.veil_transparency());
    }

    #[test]
    fn test_density_veil_thickness() {
        // From V5 Phase 1 specifications
        assert_eq!(
            Density::First.veil_thickness(),
            0.95,
            "1st density should have thickest veil"
        );
        assert_eq!(Density::Second.veil_thickness(), 0.85);
        assert_eq!(
            Density::Third.veil_thickness(),
            0.70,
            "3rd density should have thick veil"
        );
        assert_eq!(Density::Fourth.veil_thickness(), 0.40);
        assert_eq!(Density::Fifth.veil_thickness(), 0.20);
        assert_eq!(Density::Sixth.veil_thickness(), 0.10);
        assert_eq!(
            Density::Seventh.veil_thickness(),
            0.0,
            "7th density should have no veil"
        );
        assert_eq!(
            Density::Eighth.veil_thickness(),
            0.0,
            "8th density should have no veil"
        );

        // Verify thickness decreases with density
        assert!(Density::First.veil_thickness() > Density::Second.veil_thickness());
        assert!(Density::Third.veil_thickness() > Density::Fourth.veil_thickness());
        assert!(Density::Fifth.veil_thickness() > Density::Sixth.veil_thickness());
    }

    #[test]
    fn test_density_evolution_direction() {
        // From V5 Phase 1 specifications
        assert_eq!(
            Density::First.evolution_direction(),
            0.1,
            "1st density should have slow evolution"
        );
        assert_eq!(Density::Second.evolution_direction(), 0.2);
        assert_eq!(Density::Third.evolution_direction(), 0.3);
        assert_eq!(Density::Fourth.evolution_direction(), 0.5);
        assert_eq!(Density::Fifth.evolution_direction(), 0.7);
        assert_eq!(
            Density::Sixth.evolution_direction(),
            1.0,
            "6th density should have fast evolution"
        );
        assert_eq!(Density::Seventh.evolution_direction(), 1.5);
        assert_eq!(Density::Eighth.evolution_direction(), 2.0);

        // Verify evolution direction increases with density
        assert!(Density::First.evolution_direction() < Density::Second.evolution_direction());
        assert!(Density::Third.evolution_direction() < Density::Fourth.evolution_direction());
        assert!(Density::Fifth.evolution_direction() < Density::Sixth.evolution_direction());
    }

    #[test]
    fn test_density_inverse_relationship() {
        // Transparency and thickness should have inverse relationship
        for i in 1..=8 {
            let density = Density::from_u8(i);
            let sum = density.veil_transparency() + density.veil_thickness();
            // Sum should be approximately 1.0 for most densities
            // 1st: 0.0 + 0.95 = 0.95
            // 2nd: 0.1 + 0.85 = 0.95
            // 3rd: 0.3 + 0.70 = 1.0
            // 4th: 0.5 + 0.40 = 0.9
            // 5th: 0.7 + 0.20 = 0.9
            // 6th: 0.9 + 0.10 = 1.0
            // 7th: 1.0 + 0.0 = 1.0
            // 8th: 1.0 + 0.0 = 1.0
            assert!(
                (0.899..=1.001).contains(&sum),
                "Density {:?}: transparency + thickness = {}, expected ~1.0",
                density,
                sum
            );
        }
    }

    #[test]
    fn test_density_hash_implementation() {
        // Verify Density implements Hash trait (needed for spectrum_position.rs)
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(Density::First);
        set.insert(Density::Second);
        set.insert(Density::Third);

        assert_eq!(set.len(), 3, "Density should be hashable");
        assert!(set.contains(&Density::First));
        assert!(set.contains(&Density::Second));
        assert!(set.contains(&Density::Third));
        assert!(!set.contains(&Density::Fourth));
    }

    #[test]
    fn test_density_monotonic_properties() {
        // All three methods should have monotonic relationships with density
        let densities = [
            Density::First,
            Density::Second,
            Density::Third,
            Density::Fourth,
            Density::Fifth,
            Density::Sixth,
            Density::Seventh,
            Density::Eighth,
        ];

        let mut prev_transparency = densities[0].veil_transparency();
        let mut prev_thickness = densities[0].veil_thickness();
        let mut prev_direction = densities[0].evolution_direction();

        for density in densities.iter().skip(1) {
            let transparency = density.veil_transparency();
            let thickness = density.veil_thickness();
            let direction = density.evolution_direction();

            // Transparency should increase or stay same
            assert!(
                transparency >= prev_transparency,
                "Veil transparency should increase: {:?} -> {:?}",
                prev_transparency,
                transparency
            );
            prev_transparency = transparency;

            // Thickness should decrease or stay same
            assert!(
                thickness <= prev_thickness,
                "Veil thickness should decrease: {:?} -> {:?}",
                prev_thickness,
                thickness
            );
            prev_thickness = thickness;

            // Evolution direction should increase
            assert!(
                direction > prev_direction,
                "Evolution direction should increase: {:?} -> {:?}",
                prev_direction,
                direction
            );
            prev_direction = direction;
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

// Additional Density methods
impl Density {
    /// Get the next density level
    pub fn next(&self) -> Self {
        match self {
            Density::First => Density::Second,
            Density::Second => Density::Third,
            Density::Third => Density::Fourth,
            Density::Fourth => Density::Fifth,
            Density::Fifth => Density::Sixth,
            Density::Sixth => Density::Seventh,
            Density::Seventh => Density::Eighth,
            Density::Eighth => Density::Eighth, // No further transition
        }
    }
}
