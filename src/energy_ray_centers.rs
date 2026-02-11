//! Energy Ray Centers Module
//!
//! This module implements the seven energy ray centers that serve as the interface between
//! involution (energy flowing from the creator) and evolution (entities processing catalysts).
//!
//! The energy ray centers progressively solidify energy from the ethereal to the material:
//! - Love/Light (potential energy) → Light/Love (kinetic energy) → Violet → Indigo → Blue → Green → Yellow → Orange → Red (chemical body)
//!
//! Each ray center corresponds to a subtle body that the entity develops and perfects.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Represents a single ray that is packed into the Red Ray
///
/// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.7
/// "The Red Ray is NOT 'low' or 'simple.' It is the Densest Packing of the Violet Ray."
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackedRay {
    /// The ray type that is packed
    pub ray_type: RayCenter,

    /// Compression ratio (0.0 to 1.0, where 1.0 = fully compressed)
    /// Higher rays are more compressed when packed into Red
    pub compression_ratio: f64,

    /// Whether this ray is accessible for unpacking
    pub is_accessible: bool,

    /// The energy potential contained in this packed ray
    pub contained_potential: f64,
}

impl PackedRay {
    /// Creates a new packed ray
    pub fn new(ray_type: RayCenter, compression_ratio: f64) -> Self {
        // Higher rays have higher compression ratios
        let actual_compression = compression_ratio.min(1.0).max(0.0);

        PackedRay {
            ray_type,
            compression_ratio: actual_compression,
            is_accessible: false, // Initially inaccessible
            contained_potential: 100.0 * actual_compression, // Base potential scaled by compression
        }
    }

    /// Unpacks this ray, returning its contained potential
    pub fn unpack(&mut self) -> f64 {
        if !self.is_accessible {
            return 0.0;
        }

        let potential = self.contained_potential;
        self.contained_potential = 0.0; // Potential is now unpacked
        potential
    }

    /// Makes this ray accessible for unpacking
    pub fn make_accessible(&mut self) {
        self.is_accessible = true;
    }

    /// Returns whether this ray is fully compressed
    pub fn is_fully_compressed(&self) -> bool {
        self.compression_ratio >= 0.95
    }
}

/// Represents all higher rays packed into the Red Ray
///
/// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.7
/// "The Co-Creator appears at the 'Bottom of the Wave' with the entire architecture folded within."
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackedRays {
    /// All higher rays packed into Red
    pub orange_ray: PackedRay,
    pub yellow_ray: PackedRay,
    pub green_ray: PackedRay,
    pub blue_ray: PackedRay,
    pub indigo_ray: PackedRay,
    pub violet_ray: PackedRay,
}

impl PackedRays {
    /// Creates a new PackedRays structure with all higher rays packed
    pub fn new() -> Self {
        // Compression ratios increase with ray level
        // Violet is most compressed, Orange is least compressed
        PackedRays {
            orange_ray: PackedRay::new(RayCenter::Orange, 0.50),
            yellow_ray: PackedRay::new(RayCenter::Yellow, 0.60),
            green_ray: PackedRay::new(RayCenter::Green, 0.70),
            blue_ray: PackedRay::new(RayCenter::Blue, 0.80),
            indigo_ray: PackedRay::new(RayCenter::Indigo, 0.90),
            violet_ray: PackedRay::new(RayCenter::Violet, 0.99),
        }
    }

    /// Returns all packed rays as a vector
    pub fn all_rays(&self) -> Vec<&PackedRay> {
        vec![
            &self.orange_ray,
            &self.yellow_ray,
            &self.green_ray,
            &self.blue_ray,
            &self.indigo_ray,
            &self.violet_ray,
        ]
    }

    /// Returns the total potential contained in all packed rays
    pub fn total_contained_potential(&self) -> f64 {
        self.all_rays()
            .iter()
            .map(|ray| ray.contained_potential)
            .sum()
    }

    /// Returns the average compression ratio
    pub fn average_compression_ratio(&self) -> f64 {
        let rays = self.all_rays();
        let total: f64 = rays.iter().map(|ray| ray.compression_ratio).sum();
        total / rays.len() as f64
    }

    /// Makes a specific ray accessible for unpacking
    pub fn make_ray_accessible(&mut self, ray_type: RayCenter) -> bool {
        match ray_type {
            RayCenter::Orange => {
                self.orange_ray.make_accessible();
                true
            }
            RayCenter::Yellow => {
                self.yellow_ray.make_accessible();
                true
            }
            RayCenter::Green => {
                self.green_ray.make_accessible();
                true
            }
            RayCenter::Blue => {
                self.blue_ray.make_accessible();
                true
            }
            RayCenter::Indigo => {
                self.indigo_ray.make_accessible();
                true
            }
            RayCenter::Violet => {
                self.violet_ray.make_accessible();
                true
            }
            RayCenter::Red => false, // Red cannot be unpacked from itself
        }
    }
}

/// Represents the type of ray center with densest packing support
///
/// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.7
/// "The Red Ray is NOT 'low' or 'simple.' It is the Densest Packing of the Violet Ray."
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RayCenterType {
    /// Red Ray is the Densest Packing of Violet Ray
    /// It contains all higher rays folded within
    Red {
        /// All higher rays packed into Red
        packed_rays: PackedRays,

        /// How much Violet is compressed (0.0 to 1.0)
        compression_ratio: f64,

        /// Whether the entity has begun unpacking
        has_begun_unpacking: bool,
    },

    /// Other ray centers (Orange through Violet)
    Orange,
    Yellow,
    Green,
    Blue,
    Indigo,
    Violet,
}

impl RayCenterType {
    /// Creates a new Red Ray type with packed rays
    pub fn new_red() -> Self {
        RayCenterType::Red {
            packed_rays: PackedRays::new(),
            compression_ratio: 0.99, // Nearly full compression
            has_begun_unpacking: false,
        }
    }

    /// Unpacks the compressed rays from Red
    /// Returns a vector of (ray_type, potential) tuples
    pub fn unpack(&mut self) -> Vec<(RayCenter, f64)> {
        match self {
            RayCenterType::Red {
                packed_rays,
                has_begun_unpacking,
                ..
            } => {
                *has_begun_unpacking = true;

                let mut unpacked = Vec::new();

                // Unpack each accessible ray
                if packed_rays.orange_ray.is_accessible {
                    unpacked.push((RayCenter::Orange, packed_rays.orange_ray.unpack()));
                }
                if packed_rays.yellow_ray.is_accessible {
                    unpacked.push((RayCenter::Yellow, packed_rays.yellow_ray.unpack()));
                }
                if packed_rays.green_ray.is_accessible {
                    unpacked.push((RayCenter::Green, packed_rays.green_ray.unpack()));
                }
                if packed_rays.blue_ray.is_accessible {
                    unpacked.push((RayCenter::Blue, packed_rays.blue_ray.unpack()));
                }
                if packed_rays.indigo_ray.is_accessible {
                    unpacked.push((RayCenter::Indigo, packed_rays.indigo_ray.unpack()));
                }
                if packed_rays.violet_ray.is_accessible {
                    unpacked.push((RayCenter::Violet, packed_rays.violet_ray.unpack()));
                }

                unpacked
            }
            _ => vec![], // Other rays cannot unpack
        }
    }

    /// Returns whether this is a Red Ray with packed rays
    pub fn is_red_with_packed_rays(&self) -> bool {
        matches!(self, RayCenterType::Red { .. })
    }

    /// Returns the total potential contained in packed rays
    pub fn contained_potential(&self) -> f64 {
        match self {
            RayCenterType::Red { packed_rays, .. } => packed_rays.total_contained_potential(),
            _ => 0.0,
        }
    }

    /// Returns the compression ratio
    pub fn compression_ratio(&self) -> f64 {
        match self {
            RayCenterType::Red {
                compression_ratio, ..
            } => *compression_ratio,
            _ => 0.0,
        }
    }

    /// Makes a specific ray accessible for unpacking
    pub fn make_ray_accessible(&mut self, ray_type: RayCenter) -> bool {
        match self {
            RayCenterType::Red { packed_rays, .. } => packed_rays.make_ray_accessible(ray_type),
            _ => false,
        }
    }

    /// Returns whether unpacking has begun
    pub fn has_begun_unpacking(&self) -> bool {
        match self {
            RayCenterType::Red {
                has_begun_unpacking,
                ..
            } => *has_begun_unpacking,
            _ => false,
        }
    }

    /// Returns the number of accessible packed rays
    pub fn accessible_ray_count(&self) -> usize {
        match self {
            RayCenterType::Red { packed_rays, .. } => packed_rays
                .all_rays()
                .iter()
                .filter(|ray| ray.is_accessible)
                .count(),
            _ => 0,
        }
    }

    /// Returns a description of this ray center type
    pub fn description(&self) -> &str {
        match self {
            RayCenterType::Red { .. } => {
                "Red Ray - Densest Packing of Violet Ray, containing all higher rays folded within"
            }
            RayCenterType::Orange => "Orange Ray - Emotional and personal complex",
            RayCenterType::Yellow => "Yellow Ray - Solar plexus and ego center",
            RayCenterType::Green => "Green Ray - Heart center and universal love",
            RayCenterType::Blue => "Blue Ray - Communication and self-expression",
            RayCenterType::Indigo => "Indigo Ray - Gateway to intelligent infinity",
            RayCenterType::Violet => "Violet Ray - Total expression of entity's vibratory complex",
        }
    }
}

/// Represents the seven energy ray centers (chakras)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RayCenter {
    Red = 1,
    Orange = 2,
    Yellow = 3,
    Green = 4,
    Blue = 5,
    Indigo = 6,
    Violet = 7,
}

impl RayCenter {
    /// Returns all ray centers in order from red to violet (1 to 7)
    pub fn all() -> Vec<RayCenter> {
        vec![
            RayCenter::Red,
            RayCenter::Orange,
            RayCenter::Yellow,
            RayCenter::Green,
            RayCenter::Blue,
            RayCenter::Indigo,
            RayCenter::Violet,
        ]
    }

    /// Returns ray centers in order from violet to red (7 to 1) for involution
    pub fn involution_order() -> Vec<RayCenter> {
        vec![
            RayCenter::Violet,
            RayCenter::Indigo,
            RayCenter::Blue,
            RayCenter::Green,
            RayCenter::Yellow,
            RayCenter::Orange,
            RayCenter::Red,
        ]
    }

    /// Returns ray centers in order from red to violet (1 to 7) for evolution
    pub fn evolution_order() -> Vec<RayCenter> {
        RayCenter::all()
    }

    /// Returns the name of the ray center
    pub fn name(&self) -> &str {
        match self {
            RayCenter::Red => "Red Ray",
            RayCenter::Orange => "Orange Ray",
            RayCenter::Yellow => "Yellow Ray",
            RayCenter::Green => "Green Ray",
            RayCenter::Blue => "Blue Ray",
            RayCenter::Indigo => "Indigo Ray",
            RayCenter::Violet => "Violet Ray",
        }
    }

    /// Returns the associated energy ray body
    pub fn associated_body(&self) -> EnergyRayBody {
        match self {
            RayCenter::Red => EnergyRayBody::Chemical,
            RayCenter::Orange => EnergyRayBody::Physical,
            RayCenter::Yellow => EnergyRayBody::PhysicalVehicle,
            RayCenter::Green => EnergyRayBody::Astral,
            RayCenter::Blue => EnergyRayBody::Devachanic,
            RayCenter::Indigo => EnergyRayBody::Etheric,
            RayCenter::Violet => EnergyRayBody::Buddha,
        }
    }

    /// Returns the crystalline structure description
    pub fn crystalline_structure(&self) -> &str {
        match self {
            RayCenter::Red => "Spoked wheel shape",
            RayCenter::Orange => "Flower shape containing three petals",
            RayCenter::Yellow => "Rounded shape, many faceted, like a star",
            RayCenter::Green => "Lotus shape, number of points dependent on strength",
            RayCenter::Blue => "Capable of one hundred facets, great flashing brilliance",
            RayCenter::Indigo => "Basic triangular or three-petaled shape",
            RayCenter::Violet => "Thousand-petaled, least variable",
        }
    }

    /// Returns the primary function of the ray center
    pub fn primary_function(&self) -> &str {
        match self {
            RayCenter::Red => "Foundation and survival energy center",
            RayCenter::Orange => "Emotional and personal complex",
            RayCenter::Yellow => "Solar plexus and ego center",
            RayCenter::Green => "Heart center and universal love",
            RayCenter::Blue => "Communication and self-expression",
            RayCenter::Indigo => "Gateway to intelligent infinity",
            RayCenter::Violet => "Total expression of entity's vibratory complex",
        }
    }

    /// Returns whether this is a primary ray (red, yellow, or blue)
    pub fn is_primary(&self) -> bool {
        matches!(self, RayCenter::Red | RayCenter::Yellow | RayCenter::Blue)
    }

    /// Returns whether this ray can contain packed rays
    ///
    /// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.7
    /// "The Red Ray is NOT 'low' or 'simple.' It is the Densest Packing of the Violet Ray."
    pub fn can_contain_packed_rays(&self) -> bool {
        matches!(self, RayCenter::Red)
    }

    /// Returns the compression level if this ray is packed
    /// Returns None for non-Red rays
    pub fn compression_level(&self) -> Option<f64> {
        match self {
            RayCenter::Red => Some(0.99), // Nearly full compression
            _ => None,
        }
    }

    /// Returns the number of rays contained within this ray
    /// Red contains 6 higher rays, others contain 0
    pub fn contained_ray_count(&self) -> usize {
        match self {
            RayCenter::Red => 6, // Orange, Yellow, Green, Blue, Indigo, Violet
            _ => 0,
        }
    }

    /// Returns whether this ray is the densest packing
    pub fn is_densest_packing(&self) -> bool {
        matches!(self, RayCenter::Red)
    }

    /// Returns the rays that are contained within this ray
    pub fn contained_rays(&self) -> Vec<RayCenter> {
        match self {
            RayCenter::Red => vec![
                RayCenter::Orange,
                RayCenter::Yellow,
                RayCenter::Green,
                RayCenter::Blue,
                RayCenter::Indigo,
                RayCenter::Violet,
            ],
            _ => vec![],
        }
    }

    /// Returns a philosophical description of this ray's role
    ///
    /// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.7
    pub fn philosophical_description(&self) -> &str {
        match self {
            RayCenter::Red => {
                "Red Ray is the Densest Packing of Violet Ray. The Co-Creator appears at the 'Bottom of the Wave' with the entire architecture folded within."
            }
            RayCenter::Orange => {
                "Orange Ray is the emotional and personal complex, emerging from Red's packed potential."
            }
            RayCenter::Yellow => {
                "Yellow Ray is the solar plexus and ego center, emerging from Red's packed potential."
            }
            RayCenter::Green => {
                "Green Ray is the heart center and universal love, emerging from Red's packed potential."
            }
            RayCenter::Blue => {
                "Blue Ray is the communication and self-expression center, emerging from Red's packed potential."
            }
            RayCenter::Indigo => {
                "Indigo Ray is the gateway to intelligent infinity, emerging from Red's packed potential."
            }
            RayCenter::Violet => {
                "Violet Ray is the total expression of entity's vibratory complex, which was packed into Red during Involution."
            }
        }
    }

    /// Returns whether this ray is "low" or "simple" (philosophically incorrect)
    /// This method returns false for Red Ray, demonstrating it is NOT low or simple
    pub fn is_low_or_simple(&self) -> bool {
        match self {
            RayCenter::Red => false, // Red is NOT low or simple - it's densest packing
            _ => true,               // Other rays are simpler relative to Red
        }
    }

    /// Returns the density level of this ray (1 = most material, 7 = most ethereal)
    /// Red has the highest density of packed potential
    pub fn density_of_packed_potential(&self) -> f64 {
        match self {
            RayCenter::Red => 6.0, // Contains 6 rays packed within
            _ => 0.0,
        }
    }
}

impl fmt::Display for RayCenter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Represents the seven energy ray bodies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnergyRayBody {
    Chemical,        // Red ray - Unconstructed material, elemental body
    Physical,        // Orange ray - Body without self-awareness
    PhysicalVehicle, // Yellow ray - Body with mind/body/spirit characteristics
    Astral,          // Green ray - Lighter body packed densely with life
    Devachanic,      // Blue ray - Light body
    Etheric,         // Indigo ray - Gateway body where form is substance
    Buddha,          // Violet ray - Complete body, wholeness close to unity
}

impl EnergyRayBody {
    /// Returns the description of the energy ray body
    pub fn description(&self) -> &str {
        match self {
            EnergyRayBody::Chemical => "Unconstructed material of the body, the elemental body without form",
            EnergyRayBody::Physical => "Body formed without self-awareness, the body in the womb before spirit/mind complex enters",
            EnergyRayBody::PhysicalVehicle => "The physical body in which one experiences catalyst, with mind/body/spirit characteristics",
            EnergyRayBody::Astral => "Lighter body packed densely with life, may be seen in séance as ectoplasm",
            EnergyRayBody::Devachanic => "Light body, known in various traditions with many names",
            EnergyRayBody::Etheric => "Gateway body where form is substance, can mold itself as desired",
            EnergyRayBody::Buddha => "Complete body, representing wholeness close to unity with all that exists",
        }
    }

    /// Returns the associated ray center
    pub fn associated_ray_center(&self) -> RayCenter {
        match self {
            EnergyRayBody::Chemical => RayCenter::Red,
            EnergyRayBody::Physical => RayCenter::Orange,
            EnergyRayBody::PhysicalVehicle => RayCenter::Yellow,
            EnergyRayBody::Astral => RayCenter::Green,
            EnergyRayBody::Devachanic => RayCenter::Blue,
            EnergyRayBody::Etheric => RayCenter::Indigo,
            EnergyRayBody::Buddha => RayCenter::Violet,
        }
    }

    /// Returns the solidification level (1 = most material, 7 = most ethereal)
    pub fn solidification_level(&self) -> u8 {
        match self {
            EnergyRayBody::Chemical => 1,
            EnergyRayBody::Physical => 2,
            EnergyRayBody::PhysicalVehicle => 3,
            EnergyRayBody::Astral => 4,
            EnergyRayBody::Devachanic => 5,
            EnergyRayBody::Etheric => 6,
            EnergyRayBody::Buddha => 7,
        }
    }
}

/// Represents the type of energy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnergyType {
    LoveLight, // Potential energy, intelligent infinity in action
    LightLove, // Kinetic energy, the manifestation, building block of matter
}

/// Represents which complex a sub-color belongs to
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplexAffiliation {
    Mind,
    Body,
    Spirit,
}

/// Cross-Coupling at center level
///
/// Knowledge Base Reference: REFACTOR_ROADMAP_V3.md Phase 5
/// "How Mind, Body, Spirit aspects are interacting at this level"
///
/// PHASE 5 REFACTOR:
/// Tracks cross-coupling at the energy center level, showing how
/// Mind/Body/Spirit aspects are integrated at each center.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CenterCoupling {
    /// Mind aspect coupling at this center (0.0 to 1.0)
    pub mind_aspect: f64,

    /// Body aspect coupling at this center (0.0 to 1.0)
    pub body_aspect: f64,

    /// Spirit aspect coupling at this center (0.0 to 1.0)
    pub spirit_aspect: f64,
}

impl CenterCoupling {
    /// Creates a new center coupling with default values
    pub fn new() -> Self {
        CenterCoupling {
            mind_aspect: 0.5,
            body_aspect: 0.5,
            spirit_aspect: 0.5,
        }
    }

    /// Creates a center coupling with specific values
    pub fn with_values(mind: f64, body: f64, spirit: f64) -> Self {
        CenterCoupling {
            mind_aspect: mind.clamp(0.0, 1.0),
            body_aspect: body.clamp(0.0, 1.0),
            spirit_aspect: spirit.clamp(0.0, 1.0),
        }
    }

    /// Returns the overall balance of this center's coupling
    pub fn overall_balance(&self) -> f64 {
        (self.mind_aspect + self.body_aspect + self.spirit_aspect) / 3.0
    }

    /// Updates coupling based on Mind/Body/Spirit aspects
    pub fn update_from_aspects(&mut self, mind: f64, body: f64, spirit: f64) {
        self.mind_aspect = mind.clamp(0.0, 1.0);
        self.body_aspect = body.clamp(0.0, 1.0);
        self.spirit_aspect = spirit.clamp(0.0, 1.0);
    }
}

impl Default for CenterCoupling {
    fn default() -> Self {
        Self::new()
    }
}

/// Energy Center as an INTERFACE to unlock the seed's potential
///
/// Knowledge Base Reference: REFACTOR_ROADMAP_V3.md Phase 5
/// "Energy Centers are INTERFACES, not containers"
/// "They don't 'contain' anything—they 'unlock' what's folded within"
///
/// PHASE 5 REFACTOR:
/// EnergyCenter is an interface that unlocks the HolographicSeed's potential,
/// not a container that stores energy. Each center has 7 sub-colors that
/// interface to specific archetypes.
///
/// Key Principles:
/// 1. Energy Centers are INTERFACES, not containers
/// 2. Each center has 7 sub-colors (internal differentiation)
/// 3. Development is SPIRAL, not linear (Free Will enables non-linear leaps)
/// 4. Free Will (Archetype 22) is the activation mechanism
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyCenter {
    /// The ray center type
    pub ray_center: RayCenter,

    /// The center's activation level (0.0 to 1.0)
    /// Tracks how much of the seed's potential is unlocked
    pub activation: f64,

    /// The center's balance (how well Mind/Body/Spirit are integrated)
    pub balance: f64,

    /// The center's sub-colors (internal differentiation)
    /// Each energy center has 7 sub-colors (Ra, Session 51)
    pub sub_colors: [SubColor; 7],

    /// The center's cross-coupling state
    /// How Mind, Body, Spirit aspects are interacting at this level
    pub coupling: CenterCoupling,

    /// Associated archetype indices for this center
    ///
    /// Knowledge Base Reference: REFACTOR_ROADMAP_V3.md Phase 5
    /// Energy Center to Archetype Mapping:
    /// Red (Center 1): Body Complex (A8)
    /// Orange (Center 2): Body Complex (A9)
    /// Yellow (Center 3): Mind Complex (A3)
    /// Green (Center 4): Bridge (All complexes)
    /// Blue (Center 5): Spirit Complex (A16)
    /// Indigo (Center 6): Spirit Complex (A17)
    /// Violet (Center 7): Spirit Complex (A21)
    pub archetype_mapping: Vec<usize>,
}

impl EnergyCenter {
    /// Creates a new energy center as an interface
    ///
    /// PHASE 5 REFACTOR:
    /// Initializes the center as an interface that unlocks potential,
    /// with appropriate sub-colors and archetype mappings.
    pub fn new(ray_center: RayCenter) -> Self {
        // Determine archetype mappings for this center
        let archetype_mapping = Self::get_archetype_mapping(ray_center);

        // Determine sub-color affiliations
        let sub_colors = Self::create_sub_colors(ray_center, &archetype_mapping);

        EnergyCenter {
            ray_center,
            activation: 0.0,
            balance: 0.5,
            sub_colors,
            coupling: CenterCoupling::new(),
            archetype_mapping,
        }
    }

    /// Creates sub-colors for this energy center
    fn create_sub_colors(ray_center: RayCenter, archetype_mapping: &[usize]) -> [SubColor; 7] {
        // Determine complex affiliation based on ray center
        let affiliations: Vec<ComplexAffiliation> = match ray_center {
            RayCenter::Red => vec![
                ComplexAffiliation::Body,
                ComplexAffiliation::Body,
                ComplexAffiliation::Body,
                ComplexAffiliation::Body,
                ComplexAffiliation::Body,
                ComplexAffiliation::Body,
                ComplexAffiliation::Body,
            ],
            RayCenter::Orange => vec![
                ComplexAffiliation::Body,
                ComplexAffiliation::Body,
                ComplexAffiliation::Body,
                ComplexAffiliation::Mind,
                ComplexAffiliation::Mind,
                ComplexAffiliation::Mind,
                ComplexAffiliation::Mind,
            ],
            RayCenter::Yellow => vec![
                ComplexAffiliation::Body,
                ComplexAffiliation::Body,
                ComplexAffiliation::Mind,
                ComplexAffiliation::Mind,
                ComplexAffiliation::Mind,
                ComplexAffiliation::Spirit,
                ComplexAffiliation::Spirit,
            ],
            RayCenter::Green => vec![
                ComplexAffiliation::Body,
                ComplexAffiliation::Mind,
                ComplexAffiliation::Mind,
                ComplexAffiliation::Spirit,
                ComplexAffiliation::Spirit,
                ComplexAffiliation::Spirit,
                ComplexAffiliation::Spirit,
            ],
            RayCenter::Blue => vec![
                ComplexAffiliation::Mind,
                ComplexAffiliation::Mind,
                ComplexAffiliation::Mind,
                ComplexAffiliation::Spirit,
                ComplexAffiliation::Spirit,
                ComplexAffiliation::Spirit,
                ComplexAffiliation::Spirit,
            ],
            RayCenter::Indigo => vec![
                ComplexAffiliation::Mind,
                ComplexAffiliation::Spirit,
                ComplexAffiliation::Spirit,
                ComplexAffiliation::Spirit,
                ComplexAffiliation::Spirit,
                ComplexAffiliation::Spirit,
                ComplexAffiliation::Spirit,
            ],
            RayCenter::Violet => vec![
                ComplexAffiliation::Spirit,
                ComplexAffiliation::Spirit,
                ComplexAffiliation::Spirit,
                ComplexAffiliation::Spirit,
                ComplexAffiliation::Spirit,
                ComplexAffiliation::Spirit,
                ComplexAffiliation::Spirit,
            ],
        };

        // Create sub-colors with archetype indices
        let mut sub_colors: Vec<SubColor> = Vec::new();
        for (i, affiliation) in affiliations.iter().enumerate() {
            let archetype_idx = if i < archetype_mapping.len() {
                archetype_mapping[i] as u8
            } else {
                // Default mapping if not specified
                match ray_center {
                    RayCenter::Red => 7,     // A8 (index 7)
                    RayCenter::Orange => 8,  // A9 (index 8)
                    RayCenter::Yellow => 2,  // A3 (index 2)
                    RayCenter::Green => 0,   // A1 (default)
                    RayCenter::Blue => 15,   // A16 (index 15)
                    RayCenter::Indigo => 16, // A17 (index 16)
                    RayCenter::Violet => 20, // A21 (index 20)
                }
            };
            sub_colors.push(SubColor::new(archetype_idx, *affiliation));
        }

        // Convert to array
        [
            sub_colors[0].clone(),
            sub_colors[1].clone(),
            sub_colors[2].clone(),
            sub_colors[3].clone(),
            sub_colors[4].clone(),
            sub_colors[5].clone(),
            sub_colors[6].clone(),
        ]
    }

    /// Gets archetype mapping for this center
    ///
    /// Knowledge Base Reference: REFACTOR_ROADMAP_V3.md Phase 5
    fn get_archetype_mapping(ray_center: RayCenter) -> Vec<usize> {
        match ray_center {
            // Red (Center 1): Body Complex (A8)
            RayCenter::Red => vec![7, 7, 7, 7, 7, 7, 7],
            // Orange (Center 2): Body Complex (A9)
            RayCenter::Orange => vec![8, 8, 8, 8, 8, 8, 8],
            // Yellow (Center 3): Mind Complex (A3)
            RayCenter::Yellow => vec![2, 2, 2, 2, 2, 2, 2],
            // Green (Center 4): Bridge (All complexes)
            RayCenter::Green => vec![2, 7, 8, 15, 16, 20, 21], // Mix of all complexes
            // Blue (Center 5): Spirit Complex (A16)
            RayCenter::Blue => vec![15, 15, 15, 15, 15, 15, 15],
            // Indigo (Center 6): Spirit Complex (A17)
            RayCenter::Indigo => vec![16, 16, 16, 16, 16, 16, 16],
            // Violet (Center 7): Spirit Complex (A21)
            RayCenter::Violet => vec![20, 20, 20, 20, 20, 20, 20],
        }
    }

    /// Unlock the seed's potential at this center
    ///
    /// Knowledge Base Reference: REFACTOR_ROADMAP_V3.md Phase 5
    /// "Use Free Will (Archetype 22) to activate higher potentiated centers"
    /// "This creates the spiral development pattern"
    ///
    /// PHASE 5 REFACTOR:
    /// This method implements the spiral development pattern where
    /// the entity can make a Free Will choice to activate higher centers
    /// even if lower centers are not perfectly balanced.
    ///
    /// The unlock mechanism:
    /// 1. Uses Free Will (Archetype 22) for activation
    /// 2. Supports spiral development (non-linear leaps)
    /// 3. Allows choice to balance lower centers first
    ///
    /// Arguments:
    /// * archetype_activation - The archetype activation state (for Archetype 22)
    ///
    /// Returns:
    /// The amount of potential unlocked
    pub fn unlock(&mut self, archetype_activation: f64) -> f64 {
        // Free Will (Archetype 22) determines unlock capacity
        let free_will_capacity = archetype_activation.clamp(0.0, 1.0);

        // Spiral development: can unlock higher centers without perfect lower balance
        // This allows non-linear "leaps" in consciousness

        // Base unlock amount based on Free Will capacity
        let unlock_amount = free_will_capacity * 0.1;

        // Increase activation (spiraling upward)
        self.activation = (self.activation + unlock_amount).min(1.0);

        // Update balance based on activation
        self.balance = (self.balance + unlock_amount * 0.5).min(1.0);

        // Distribute to sub-colors
        for sub_color in self.sub_colors.iter_mut() {
            sub_color.activation = (sub_color.activation + unlock_amount * 0.5).min(1.0);
            sub_color.balance = (sub_color.balance + unlock_amount * 0.3).min(1.0);
        }

        // Update coupling
        self.coupling.mind_aspect = (self.coupling.mind_aspect + unlock_amount * 0.3).min(1.0);
        self.coupling.body_aspect = (self.coupling.body_aspect + unlock_amount * 0.3).min(1.0);
        self.coupling.spirit_aspect = (self.coupling.spirit_aspect + unlock_amount * 0.3).min(1.0);

        unlock_amount
    }

    /// Balance lower centers before unlocking higher ones
    ///
    /// This represents the choice to balance lower centers first,
    /// which is the more gradual path of development.
    pub fn balance_lower_centers(&mut self, amount: f64) {
        let balance_amount = amount.clamp(0.0, 1.0);

        // Increase balance without significantly increasing activation
        self.balance = (self.balance + balance_amount * 0.2).min(1.0);

        // Balance sub-colors
        for sub_color in self.sub_colors.iter_mut() {
            sub_color.balance = (sub_color.balance + balance_amount * 0.15).min(1.0);
        }

        // Update coupling toward balance
        let avg_activation = self.sub_colors.iter().map(|sc| sc.activation).sum::<f64>() / 7.0;
        self.coupling
            .update_from_aspects(avg_activation, avg_activation, avg_activation);
    }

    /// Integrate flows (Green Ray specific)
    ///
    /// Knowledge Base Reference: REFACTOR_ROADMAP_V3.md Phase 5
    /// "Up-pouring meets in-pouring - Green Ray activation"
    /// "This is the bridge between lower and higher centers"
    ///
    /// PHASE 5 REFACTOR:
    /// Green Ray activation integrates up-pouring (Body's vital energy)
    /// and in-pouring (Spirit's intelligent energy).
    ///
    /// Arguments:
    /// * up_pouring - Body generates vital energy from survival/experience
    /// * in_pouring - Spirit generates intelligent energy from source
    pub fn integrate_flows(&mut self, up_pouring: f64, in_pouring: f64) {
        // Green Ray: Up-pouring meets in-pouring
        // This is the bridge between lower and higher centers

        // Calculate integrated activation
        let integrated = (up_pouring + in_pouring) / 2.0;

        // Update activation
        self.activation = (self.activation + integrated * 0.1).min(1.0);

        // Update balance (integration improves balance)
        let flow_balance =
            1.0 - (up_pouring - in_pouring).abs() / (up_pouring + in_pouring + 0.001);
        self.balance = (self.balance + flow_balance * 0.1).min(1.0);

        // Distribute to sub-colors
        for sub_color in self.sub_colors.iter_mut() {
            sub_color.activation = (sub_color.activation + integrated * 0.05).min(1.0);
            sub_color.balance = (sub_color.balance + flow_balance * 0.05).min(1.0);
        }

        // Update coupling
        self.coupling.mind_aspect = (self.coupling.mind_aspect + integrated * 0.05).min(1.0);
        self.coupling.body_aspect = (self.coupling.body_aspect + up_pouring * 0.05).min(1.0);
        self.coupling.spirit_aspect = (self.coupling.spirit_aspect + in_pouring * 0.05).min(1.0);
    }

    /// Partial integration (Restricted valve)
    ///
    /// Knowledge Base Reference: REFACTOR_ROADMAP_V3.md Phase 5
    /// "Partial flow - some Spirit access, but limited"
    ///
    /// PHASE 5 REFACTOR:
    /// When Mind's valve is Restricted, there is partial integration
    /// with limited Spirit access.
    pub fn partial_integration(&mut self, up_pouring: f64, in_pouring: f64) {
        // Partial flow: limited Spirit access
        let spirit_factor = 0.5; // Only 50% Spirit access

        // Calculate partial integration
        let partial = (up_pouring + (in_pouring * spirit_factor)) / 2.0;

        // Update activation (less than full integration)
        self.activation = (self.activation + partial * 0.05).min(1.0);

        // Update balance (partial integration)
        let flow_balance = 1.0
            - (up_pouring - in_pouring * spirit_factor).abs()
                / (up_pouring + in_pouring * spirit_factor + 0.001);
        self.balance = (self.balance + flow_balance * 0.05).min(1.0);

        // Distribute to sub-colors (limited Spirit influence)
        for sub_color in self.sub_colors.iter_mut() {
            sub_color.activation = (sub_color.activation + partial * 0.03).min(1.0);
            sub_color.balance = (sub_color.balance + flow_balance * 0.03).min(1.0);
        }

        // Update coupling (limited Spirit aspect)
        self.coupling.mind_aspect = (self.coupling.mind_aspect + partial * 0.03).min(1.0);
        self.coupling.body_aspect = (self.coupling.body_aspect + up_pouring * 0.05).min(1.0);
        self.coupling.spirit_aspect =
            (self.coupling.spirit_aspect + in_pouring * spirit_factor * 0.03).min(1.0);
    }

    /// Returns whether this center is fully activated
    pub fn is_fully_activated(&self) -> bool {
        self.activation >= 1.0
    }

    /// Returns whether this center is balanced
    pub fn is_balanced(&self) -> bool {
        self.balance >= 0.7
    }

    /// Returns the average activation of sub-colors
    pub fn sub_color_average_activation(&self) -> f64 {
        self.sub_colors.iter().map(|sc| sc.activation).sum::<f64>() / 7.0
    }

    /// Returns the average balance of sub-colors
    pub fn sub_color_average_balance(&self) -> f64 {
        self.sub_colors.iter().map(|sc| sc.balance).sum::<f64>() / 7.0
    }
}

/// Represents a sub-color within an energy center
///
/// Knowledge Base Reference: Energy-Ray-Centers/0. General.json
/// "Each energy center has seven sub-colors" (Ra, Session 51)
///
/// PHASE 5 REFACTOR:
/// SubColor now includes archetype_index to interface to specific archetypes
/// from the HolographicSeed's 22-Archetype structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubColor {
    /// Which archetype this sub-color interfaces to (0-21)
    ///
    /// Knowledge Base Reference: REFACTOR_ROADMAP_V3.md Phase 5
    /// "Each sub-color interfaces to a specific archetype from the 22-Archetype structure"
    pub archetype_index: u8,

    /// Sub-color activation level (0.0 to 1.0)
    pub activation: f64,

    /// Sub-color balance level (0.0 to 1.0)
    pub balance: f64,

    /// Which complex this sub-color belongs to (Mind/Body/Spirit)
    pub complex_affiliation: ComplexAffiliation,
}

impl SubColor {
    /// Creates a new sub-color with default values
    ///
    /// PHASE 5 REFACTOR:
    /// Now requires archetype_index to interface to specific archetypes
    pub fn new(archetype_index: u8, complex_affiliation: ComplexAffiliation) -> Self {
        SubColor {
            archetype_index: archetype_index % 22, // Ensure valid archetype index
            activation: 0.0,
            balance: 0.5,
            complex_affiliation,
        }
    }

    /// Activates the sub-color
    pub fn activate(&mut self, amount: f64) {
        self.activation = (self.activation + amount).min(1.0);
    }

    /// Balances the sub-color
    pub fn balance_sub_color(&mut self) {
        // Balance is influenced by activation level
        self.balance = (self.balance + self.activation * 0.1).min(1.0);
    }
}

impl EnergyType {
    /// Returns the description of the energy type
    pub fn description(&self) -> &str {
        match self {
            EnergyType::LoveLight => {
                "Love/Light - The energy source, the enabler and power giver (potential energy)"
            }
            EnergyType::LightLove => {
                "Light/Love - The manifestation, the building block of matter (kinetic energy)"
            }
        }
    }
}

/// Represents the state of an energy ray center
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RayCenterState {
    /// The ray center
    pub ray_center: RayCenter,

    /// Current activation level (0.0 to 1.0)
    pub activation_level: f64,

    /// Current rotational speed or brilliance (0.0 to 1.0)
    pub rotational_speed: f64,

    /// Current energy level (0.0 to infinity)
    pub energy_level: f64,

    /// Energy capacity (maximum energy this center can hold)
    pub energy_capacity: f64,

    /// Blockage level (0.0 = no blockage, 1.0 = complete blockage)
    pub blockage_level: f64,

    /// Balance level (0.0 to 1.0, where 1.0 is perfectly balanced)
    pub balance_level: f64,

    /// Crystalline structure formation (0.0 to 1.0)
    pub crystalline_formation: f64,

    /// The center's sub-colors (internal differentiation)
    ///
    /// Knowledge Base Reference: Energy-Ray-Centers/0. General.json
    /// "Each energy center has seven sub-colors"
    pub sub_colors: [SubColor; 7],

    /// Associated energy ray body state
    pub body_state: EnergyRayBodyState,
}

impl RayCenterState {
    /// Creates a new ray center state with default values
    pub fn new(ray_center: RayCenter) -> Self {
        let body = ray_center.associated_body();

        // Initialize sub-colors based on ray center's primary complex affiliation
        // PHASE 5 REFACTOR: Now includes archetype_index for each sub-color
        // This is a simplified distribution - in full implementation would be more nuanced
        let sub_colors: [SubColor; 7] = match ray_center {
            RayCenter::Red => [
                SubColor::new(7, ComplexAffiliation::Body), // A8
                SubColor::new(7, ComplexAffiliation::Body), // A8
                SubColor::new(7, ComplexAffiliation::Body), // A8
                SubColor::new(7, ComplexAffiliation::Body), // A8
                SubColor::new(7, ComplexAffiliation::Body), // A8
                SubColor::new(7, ComplexAffiliation::Body), // A8
                SubColor::new(7, ComplexAffiliation::Body), // A8
            ],
            RayCenter::Orange => [
                SubColor::new(8, ComplexAffiliation::Body), // A9
                SubColor::new(8, ComplexAffiliation::Body), // A9
                SubColor::new(8, ComplexAffiliation::Body), // A9
                SubColor::new(8, ComplexAffiliation::Mind), // A9
                SubColor::new(8, ComplexAffiliation::Mind), // A9
                SubColor::new(8, ComplexAffiliation::Mind), // A9
                SubColor::new(8, ComplexAffiliation::Mind), // A9
            ],
            RayCenter::Yellow => [
                SubColor::new(2, ComplexAffiliation::Body),   // A3
                SubColor::new(2, ComplexAffiliation::Body),   // A3
                SubColor::new(2, ComplexAffiliation::Mind),   // A3
                SubColor::new(2, ComplexAffiliation::Mind),   // A3
                SubColor::new(2, ComplexAffiliation::Mind),   // A3
                SubColor::new(2, ComplexAffiliation::Spirit), // A3
                SubColor::new(2, ComplexAffiliation::Spirit), // A3
            ],
            RayCenter::Green => [
                SubColor::new(2, ComplexAffiliation::Body),    // A3
                SubColor::new(7, ComplexAffiliation::Mind),    // A8
                SubColor::new(8, ComplexAffiliation::Mind),    // A9
                SubColor::new(15, ComplexAffiliation::Spirit), // A16
                SubColor::new(16, ComplexAffiliation::Spirit), // A17
                SubColor::new(20, ComplexAffiliation::Spirit), // A21
                SubColor::new(21, ComplexAffiliation::Spirit), // A22
            ],
            RayCenter::Blue => [
                SubColor::new(15, ComplexAffiliation::Mind),   // A16
                SubColor::new(15, ComplexAffiliation::Mind),   // A16
                SubColor::new(15, ComplexAffiliation::Mind),   // A16
                SubColor::new(15, ComplexAffiliation::Spirit), // A16
                SubColor::new(15, ComplexAffiliation::Spirit), // A16
                SubColor::new(15, ComplexAffiliation::Spirit), // A16
                SubColor::new(15, ComplexAffiliation::Spirit), // A16
            ],
            RayCenter::Indigo => [
                SubColor::new(16, ComplexAffiliation::Mind),   // A17
                SubColor::new(16, ComplexAffiliation::Spirit), // A17
                SubColor::new(16, ComplexAffiliation::Spirit), // A17
                SubColor::new(16, ComplexAffiliation::Spirit), // A17
                SubColor::new(16, ComplexAffiliation::Spirit), // A17
                SubColor::new(16, ComplexAffiliation::Spirit), // A17
                SubColor::new(16, ComplexAffiliation::Spirit), // A17
            ],
            RayCenter::Violet => [
                SubColor::new(20, ComplexAffiliation::Spirit), // A21
                SubColor::new(20, ComplexAffiliation::Spirit), // A21
                SubColor::new(20, ComplexAffiliation::Spirit), // A21
                SubColor::new(20, ComplexAffiliation::Spirit), // A21
                SubColor::new(20, ComplexAffiliation::Spirit), // A21
                SubColor::new(20, ComplexAffiliation::Spirit), // A21
                SubColor::new(20, ComplexAffiliation::Spirit), // A21
            ],
        };

        RayCenterState {
            ray_center,
            activation_level: 0.0,
            rotational_speed: 0.0,
            energy_level: 0.0,
            energy_capacity: 100.0,
            blockage_level: 0.0,
            balance_level: 0.5,
            crystalline_formation: 0.0,
            sub_colors,
            body_state: EnergyRayBodyState::new(body),
        }
    }

    /// Receives energy from a higher ray center during involution
    pub fn receive_energy(&mut self, amount: f64) {
        // Energy is reduced by blockage level
        let effective_amount = amount * (1.0 - self.blockage_level);

        // Add energy up to capacity
        self.energy_level = (self.energy_level + effective_amount).min(self.energy_capacity);

        // Increase activation level based on energy level
        self.activation_level = (self.energy_level / self.energy_capacity).min(1.0);

        // Increase rotational speed based on activation
        self.rotational_speed = self.activation_level;

        // Distribute energy to sub-colors
        let sub_color_share = effective_amount / 7.0;
        for sub_color in self.sub_colors.iter_mut() {
            sub_color.activate(sub_color_share);
            sub_color.balance_sub_color();
        }
    }

    /// Provides energy to a lower ray center during involution
    pub fn provide_energy(&mut self, amount: f64) -> f64 {
        // Can only provide energy we have
        let available = self.energy_level.min(amount);

        // Remove the energy
        self.energy_level -= available;

        // Update activation level
        self.activation_level = (self.energy_level / self.energy_capacity).min(1.0);

        available
    }

    /// Balances the ray center
    pub fn balance(&mut self) {
        // Balance level is influenced by activation and blockage
        self.balance_level = (self.activation_level * (1.0 - self.blockage_level)).min(1.0);

        // Update crystalline formation based on balance
        self.crystalline_formation = self.balance_level;

        // Balance all sub-colors
        for sub_color in self.sub_colors.iter_mut() {
            sub_color.balance_sub_color();
        }
    }

    /// Returns the average activation of all sub-colors
    pub fn get_sub_color_average_activation(&self) -> f64 {
        let total: f64 = self.sub_colors.iter().map(|sc| sc.activation).sum();
        total / 7.0
    }

    /// Returns the average balance of all sub-colors
    pub fn get_sub_color_average_balance(&self) -> f64 {
        let total: f64 = self.sub_colors.iter().map(|sc| sc.balance).sum();
        total / 7.0
    }

    /// Returns whether sub-colors are balanced across Mind/Body/Spirit complexes
    pub fn are_sub_colors_balanced(&self) -> bool {
        let mind_avg: f64 = self
            .sub_colors
            .iter()
            .filter(|sc| sc.complex_affiliation == ComplexAffiliation::Mind)
            .map(|sc| sc.activation)
            .sum::<f64>()
            .max(1.0); // Avoid division by zero

        let body_avg: f64 = self
            .sub_colors
            .iter()
            .filter(|sc| sc.complex_affiliation == ComplexAffiliation::Body)
            .map(|sc| sc.activation)
            .sum::<f64>()
            .max(1.0);

        let spirit_avg: f64 = self
            .sub_colors
            .iter()
            .filter(|sc| sc.complex_affiliation == ComplexAffiliation::Spirit)
            .map(|sc| sc.activation)
            .sum::<f64>()
            .max(1.0);

        // Check if all complexes are within 30% of each other
        let max_avg = mind_avg.max(body_avg).max(spirit_avg);
        let min_avg = mind_avg.min(body_avg).min(spirit_avg);

        (max_avg - min_avg) < 0.3
    }

    /// Clears blockage
    pub fn clear_blockage(&mut self, amount: f64) {
        self.blockage_level = (self.blockage_level - amount).max(0.0);
        self.balance();
    }

    /// Adds blockage
    pub fn add_blockage(&mut self, amount: f64) {
        self.blockage_level = (self.blockage_level + amount).min(1.0);
        self.balance();
    }

    /// Returns whether this ray center is blocked
    pub fn is_blocked(&self) -> bool {
        self.blockage_level > 0.0
    }

    /// Returns whether this ray center is fully activated
    pub fn is_fully_activated(&self) -> bool {
        self.activation_level >= 1.0
    }
}

/// Represents the state of an energy ray body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyRayBodyState {
    /// The energy ray body
    pub body: EnergyRayBody,

    /// Development level (0.0 to 1.0)
    pub development_level: f64,

    /// Integrity level (0.0 to 1.0)
    pub integrity_level: f64,

    /// Vitality level (0.0 to 1.0)
    pub vitality_level: f64,
}

impl EnergyRayBodyState {
    /// Creates a new energy ray body state with default values
    pub fn new(body: EnergyRayBody) -> Self {
        EnergyRayBodyState {
            body,
            development_level: 0.0,
            integrity_level: 1.0,
            vitality_level: 1.0,
        }
    }

    /// Develops the body based on energy from the associated ray center
    pub fn develop(&mut self, energy: f64) {
        // Development increases with energy
        let development_increase = energy * 0.01;
        self.development_level = (self.development_level + development_increase).min(1.0);
    }

    /// Heals the body
    pub fn heal(&mut self, amount: f64) {
        self.integrity_level = (self.integrity_level + amount).min(1.0);
        self.vitality_level = (self.vitality_level + amount).min(1.0);
    }

    /// Damages the body
    pub fn damage(&mut self, amount: f64) {
        self.integrity_level = (self.integrity_level - amount).max(0.0);
        self.vitality_level = (self.vitality_level - amount).max(0.0);
    }

    /// Returns whether the body is fully developed
    pub fn is_fully_developed(&self) -> bool {
        self.development_level >= 1.0
    }
}

/// Represents the energy transformation process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyTransformation {
    /// Current energy type
    pub energy_type: EnergyType,

    /// Transformation progress (0.0 to 1.0)
    pub transformation_progress: f64,

    /// Energy available for transformation
    pub available_energy: f64,
}

impl EnergyTransformation {
    /// Creates a new energy transformation
    pub fn new(energy_type: EnergyType, available_energy: f64) -> Self {
        EnergyTransformation {
            energy_type,
            transformation_progress: 0.0,
            available_energy,
        }
    }

    /// Transforms love/light to light/love
    pub fn transform_love_light_to_light_love(&mut self) -> f64 {
        if self.energy_type != EnergyType::LoveLight {
            return 0.0;
        }

        // Transformation converts potential to kinetic energy
        let transformed_energy = self.available_energy * 0.95; // 5% loss in transformation

        self.energy_type = EnergyType::LightLove;
        self.transformation_progress = 1.0;
        self.available_energy = transformed_energy;

        transformed_energy
    }

    /// Returns whether transformation is complete
    pub fn is_complete(&self) -> bool {
        self.transformation_progress >= 1.0
    }
}

/// Represents the progressive solidification of energy through ray centers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressiveSolidification {
    /// Current ray center in the solidification process
    pub current_ray_center: RayCenter,

    /// Solidification progress (0.0 to 1.0)
    pub progress: f64,

    /// Energy being solidified
    pub energy: f64,

    /// Solidification direction (involution or evolution)
    pub direction: SolidificationDirection,
}

/// Represents the direction of solidification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SolidificationDirection {
    /// Involution: Violet → Indigo → Blue → Green → Yellow → Orange → Red
    Involution,
    /// Evolution: Red → Orange → Yellow → Green → Blue → Indigo → Violet
    Evolution,
}

/// Represents energy flow through the centers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyFlow {
    /// Flow intensity (0.0 to infinity)
    pub intensity: f64,
    /// Flow direction
    pub direction: FlowDirection,
    /// Associated ray color
    pub ray_color: RayCenter,
}

/// Represents the direction of energy flow
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FlowDirection {
    /// Up-pouring: Body → Spirit (evolutionary flow)
    UpPouring,
    /// In-pouring: Spirit → Body (involutionary flow)
    InPouring,
}

impl EnergyFlow {
    /// Creates a new energy flow
    pub fn new(intensity: f64, direction: FlowDirection, ray_color: RayCenter) -> Self {
        EnergyFlow {
            intensity,
            direction,
            ray_color,
        }
    }

    /// Merges this flow with another flow
    pub fn merge(&self, other: EnergyFlow) -> EnergyFlow {
        let merged_intensity = self.intensity + other.intensity;
        let merged_direction = if self.intensity > other.intensity {
            self.direction
        } else {
            other.direction
        };
        let merged_ray = if self.intensity > other.intensity {
            self.ray_color
        } else {
            other.ray_color
        };

        EnergyFlow {
            intensity: merged_intensity,
            direction: merged_direction,
            ray_color: merged_ray,
        }
    }
}

/// Represents the valve state of the Mind complex
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 4.4
/// "Mind is the VALVE that regulates the flow between Body and Spirit"
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValveState {
    /// Mind is fully open - full integration of Body and Spirit
    Open,
    /// Mind is restricted - partial Spirit access
    Restricted,
    /// Mind is closed - no Spirit integration
    Closed,
}

/// Represents the result of energy integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegrationResult {
    /// Full integration occurred
    Full(EnergyFlow),
    /// Partial integration occurred
    Partial(EnergyFlow),
    /// Flow was blocked
    Blocked(EnergyFlow),
}

impl ProgressiveSolidification {
    /// Creates a new progressive solidification for involution
    pub fn new_involution(energy: f64) -> Self {
        ProgressiveSolidification {
            current_ray_center: RayCenter::Violet,
            progress: 0.0,
            energy,
            direction: SolidificationDirection::Involution,
        }
    }

    /// Creates a new progressive solidification for evolution
    pub fn new_evolution(energy: f64) -> Self {
        ProgressiveSolidification {
            current_ray_center: RayCenter::Red,
            progress: 0.0,
            energy,
            direction: SolidificationDirection::Evolution,
        }
    }

    /// Advances the solidification to the next ray center
    pub fn advance(&mut self) -> Option<RayCenter> {
        match self.direction {
            SolidificationDirection::Involution => {
                // Move from violet to red
                match self.current_ray_center {
                    RayCenter::Violet => {
                        self.current_ray_center = RayCenter::Indigo;
                        Some(RayCenter::Indigo)
                    }
                    RayCenter::Indigo => {
                        self.current_ray_center = RayCenter::Blue;
                        Some(RayCenter::Blue)
                    }
                    RayCenter::Blue => {
                        self.current_ray_center = RayCenter::Green;
                        Some(RayCenter::Green)
                    }
                    RayCenter::Green => {
                        self.current_ray_center = RayCenter::Yellow;
                        Some(RayCenter::Yellow)
                    }
                    RayCenter::Yellow => {
                        self.current_ray_center = RayCenter::Orange;
                        Some(RayCenter::Orange)
                    }
                    RayCenter::Orange => {
                        self.current_ray_center = RayCenter::Red;
                        Some(RayCenter::Red)
                    }
                    RayCenter::Red => None, // End of involution
                }
            }
            SolidificationDirection::Evolution => {
                // Move from red to violet
                match self.current_ray_center {
                    RayCenter::Red => {
                        self.current_ray_center = RayCenter::Orange;
                        Some(RayCenter::Orange)
                    }
                    RayCenter::Orange => {
                        self.current_ray_center = RayCenter::Yellow;
                        Some(RayCenter::Yellow)
                    }
                    RayCenter::Yellow => {
                        self.current_ray_center = RayCenter::Green;
                        Some(RayCenter::Green)
                    }
                    RayCenter::Green => {
                        self.current_ray_center = RayCenter::Blue;
                        Some(RayCenter::Blue)
                    }
                    RayCenter::Blue => {
                        self.current_ray_center = RayCenter::Indigo;
                        Some(RayCenter::Indigo)
                    }
                    RayCenter::Indigo => {
                        self.current_ray_center = RayCenter::Violet;
                        Some(RayCenter::Violet)
                    }
                    RayCenter::Violet => None, // End of evolution
                }
            }
        }
    }

    /// Returns whether solidification is complete
    pub fn is_complete(&self) -> bool {
        match self.direction {
            SolidificationDirection::Involution => self.current_ray_center == RayCenter::Red,
            SolidificationDirection::Evolution => self.current_ray_center == RayCenter::Violet,
        }
    }
}

/// Represents the complete energy ray center system for an entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyRayCenterSystem {
    /// The entity ID
    pub entity_id: usize,

    /// States of all seven ray centers
    pub ray_centers: HashMap<RayCenter, RayCenterState>,

    /// Current energy transformation
    pub energy_transformation: Option<EnergyTransformation>,

    /// Current progressive solidification
    pub progressive_solidification: Option<ProgressiveSolidification>,

    /// Total energy capacity of the system
    pub total_energy_capacity: f64,

    /// Total energy level of the system
    pub total_energy_level: f64,

    /// Overall balance level (0.0 to 1.0)
    pub overall_balance: f64,
}

impl EnergyRayCenterSystem {
    /// Creates a new energy ray center system for an entity
    pub fn new(entity_id: usize) -> Self {
        let mut ray_centers = HashMap::new();

        for ray_center in RayCenter::all() {
            ray_centers.insert(ray_center, RayCenterState::new(ray_center));
        }

        let total_energy_capacity: f64 = ray_centers
            .values()
            .map(|state| state.energy_capacity)
            .sum();

        EnergyRayCenterSystem {
            entity_id,
            ray_centers,
            energy_transformation: None,
            progressive_solidification: None,
            total_energy_capacity,
            total_energy_level: 0.0,
            overall_balance: 0.5,
        }
    }

    /// Receives love/light energy from the source
    pub fn receive_love_light(&mut self, amount: f64) {
        // Create energy transformation
        let mut transformation = EnergyTransformation::new(EnergyType::LoveLight, amount);

        // Transform love/light to light/love
        let transformed_energy = transformation.transform_love_light_to_light_love();

        self.energy_transformation = Some(transformation);

        // Start progressive solidification from violet
        let mut solidification = ProgressiveSolidification::new_involution(transformed_energy);

        // Distribute energy through ray centers
        self.distribute_energy_involution(&mut solidification);

        self.progressive_solidification = Some(solidification);
    }

    /// Distributes energy through ray centers during involution
    fn distribute_energy_involution(&mut self, solidification: &mut ProgressiveSolidification) {
        let mut remaining_energy = solidification.energy;

        // Distribute energy from violet to red
        for ray_center in RayCenter::involution_order() {
            if let Some(state) = self.ray_centers.get_mut(&ray_center) {
                // Each ray center receives a portion of the energy
                let energy_share = remaining_energy * 0.15; // 15% per center

                state.receive_energy(energy_share);

                remaining_energy -= energy_share;

                if remaining_energy <= 0.0 {
                    break;
                }
            }
        }
    }

    /// Provides energy from ray centers for catalyst processing
    pub fn provide_energy_for_processing(&mut self, amount: f64) -> f64 {
        let mut provided_energy = 0.0;

        // Start from red ray and work up
        for ray_center in RayCenter::evolution_order() {
            if provided_energy >= amount {
                break;
            }

            if let Some(state) = self.ray_centers.get_mut(&ray_center) {
                let needed = amount - provided_energy;
                let available = state.energy_level.min(needed);

                state.provide_energy(available);
                provided_energy += available;
            }
        }

        provided_energy
    }

    /// Evolves the energy ray center system based on experience
    pub fn evolve(&mut self, experience: f64) {
        // Experience increases energy capacity
        let capacity_increase = experience * 0.1;

        for (ray_center, state) in self.ray_centers.iter_mut() {
            // Increase energy capacity
            state.energy_capacity += capacity_increase * (*ray_center as u8 as f64 / 7.0);

            // Develop the associated body
            state.body_state.develop(experience);

            // Balance the ray center
            state.balance();
        }

        // Update total energy capacity
        self.total_energy_capacity = self
            .ray_centers
            .values()
            .map(|state| state.energy_capacity)
            .sum();

        // Update overall balance
        self.update_overall_balance();
    }

    /// Updates the overall balance
    fn update_overall_balance(&mut self) {
        let total_balance: f64 = self
            .ray_centers
            .values()
            .map(|state| state.balance_level)
            .sum();

        self.overall_balance = total_balance / 7.0;
    }

    /// Returns the current total energy level
    pub fn get_total_energy_level(&self) -> f64 {
        self.ray_centers
            .values()
            .map(|state| state.energy_level)
            .sum()
    }

    /// Returns the state of a specific ray center
    pub fn get_ray_center_state(&self, ray_center: RayCenter) -> Option<&RayCenterState> {
        self.ray_centers.get(&ray_center)
    }

    /// Returns a mutable reference to a specific ray center state
    pub fn get_ray_center_state_mut(
        &mut self,
        ray_center: RayCenter,
    ) -> Option<&mut RayCenterState> {
        self.ray_centers.get_mut(&ray_center)
    }

    /// Returns whether the entity has a balanced energy system
    pub fn is_balanced(&self) -> bool {
        self.overall_balance >= 0.7
    }

    /// Returns whether the entity is ready for harvest
    pub fn is_ready_for_harvest(&self) -> bool {
        // Violet ray must be active and system must be balanced
        if let Some(violet_state) = self.ray_centers.get(&RayCenter::Violet) {
            return violet_state.activation_level >= 0.8 && self.overall_balance >= 0.7;
        }
        false
    }

    /// Integrates flows through a specific ray center
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 4.4
    /// "Mind is the VALVE that regulates the flow between Body and Spirit"
    pub fn integrate_flows(
        &mut self,
        ray_center: RayCenter,
        up_pouring: EnergyFlow,
        in_pouring: EnergyFlow,
        valve_state: ValveState,
    ) -> IntegrationResult {
        let state = self.ray_centers.get_mut(&ray_center);
        if state.is_none() {
            return IntegrationResult::Blocked(up_pouring);
        }

        let state = state.unwrap();

        match valve_state {
            ValveState::Open => {
                // Full integration - up-pouring meets in-pouring
                let integrated = up_pouring.merge(in_pouring);
                state.activation_level =
                    (state.activation_level + integrated.intensity * 0.1).min(1.0);
                state.receive_energy(integrated.intensity);
                IntegrationResult::Full(integrated)
            }
            ValveState::Restricted => {
                // Partial integration - some Spirit access
                let restriction_factor = 0.5;
                let partial = EnergyFlow {
                    intensity: in_pouring.intensity * restriction_factor,
                    direction: in_pouring.direction,
                    ray_color: in_pouring.ray_color,
                };
                let integrated = up_pouring.merge(partial);
                state.activation_level =
                    (state.activation_level + integrated.intensity * 0.05).min(1.0);
                state.receive_energy(integrated.intensity * 0.5);
                IntegrationResult::Partial(integrated)
            }
            ValveState::Closed => {
                // No integration - Spirit remains as potential
                // Up-pouring accumulates as unprocessed energy
                state.add_blockage(0.1);
                IntegrationResult::Blocked(up_pouring)
            }
        }
    }

    /// Processes energy flow through the system following involution or evolution pattern
    pub fn process_energy_flow(
        &mut self,
        flow: EnergyFlow,
        direction: SolidificationDirection,
    ) -> Vec<IntegrationResult> {
        let mut results = Vec::new();
        let centers: Vec<RayCenter> = match direction {
            SolidificationDirection::Involution => RayCenter::involution_order(),
            SolidificationDirection::Evolution => RayCenter::evolution_order(),
        };

        for center in centers {
            let valve_state = self.determine_valve_state(center, flow.clone());

            // Create up-pouring and in-pouring based on direction
            let (up_pouring, in_pouring) = if direction == SolidificationDirection::Evolution {
                // Evolution: Body (up) meets Spirit (in)
                (
                    EnergyFlow::new(flow.intensity, FlowDirection::UpPouring, center),
                    EnergyFlow::new(
                        flow.intensity * 0.8,
                        FlowDirection::InPouring,
                        RayCenter::Violet,
                    ),
                )
            } else {
                // Involution: Spirit (in) meets Body (up)
                (
                    EnergyFlow::new(
                        flow.intensity * 0.8,
                        FlowDirection::UpPouring,
                        RayCenter::Red,
                    ),
                    EnergyFlow::new(flow.intensity, FlowDirection::InPouring, center),
                )
            };

            let result = self.integrate_flows(center, up_pouring, in_pouring, valve_state);
            results.push(result);
        }

        results
    }

    /// Determines the valve state for a ray center based on its balance and activation
    fn determine_valve_state(&self, ray_center: RayCenter, _flow: EnergyFlow) -> ValveState {
        if let Some(state) = self.ray_centers.get(&ray_center) {
            if state.balance_level < 0.4 {
                return ValveState::Closed;
            } else if state.balance_level < 0.7 {
                return ValveState::Restricted;
            }
        }
        ValveState::Open
    }

    /// Returns the coupling state across Mind/Body/Spirit complexes
    pub fn get_coupling_state(&self) -> CouplingState {
        let mut mind_sum = 0.0;
        let mut body_sum = 0.0;
        let mut spirit_sum = 0.0;
        let mut mind_count = 0.0;
        let mut body_count = 0.0;
        let mut spirit_count = 0.0;

        for state in self.ray_centers.values() {
            for sub_color in state.sub_colors.iter() {
                match sub_color.complex_affiliation {
                    ComplexAffiliation::Mind => {
                        mind_sum += sub_color.activation;
                        mind_count += 1.0;
                    }
                    ComplexAffiliation::Body => {
                        body_sum += sub_color.activation;
                        body_count += 1.0;
                    }
                    ComplexAffiliation::Spirit => {
                        spirit_sum += sub_color.activation;
                        spirit_count += 1.0;
                    }
                }
            }
        }

        let mind_avg = if mind_count > 0.0 {
            mind_sum / mind_count
        } else {
            0.0
        };
        let body_avg = if body_count > 0.0 {
            body_sum / body_count
        } else {
            0.0
        };
        let spirit_avg = if spirit_count > 0.0 {
            spirit_sum / spirit_count
        } else {
            0.0
        };

        CouplingState {
            mind_activation: mind_avg,
            body_activation: body_avg,
            spirit_activation: spirit_avg,
            overall_coupling: (mind_avg + body_avg + spirit_avg) / 3.0,
        }
    }
}

/// Represents the coupling state between Mind/Body/Spirit complexes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouplingState {
    /// Average activation across Mind complex sub-colors
    pub mind_activation: f64,
    /// Average activation across Body complex sub-colors
    pub body_activation: f64,
    /// Average activation across Spirit complex sub-colors
    pub spirit_activation: f64,
    /// Overall coupling level (0.0 to 1.0)
    pub overall_coupling: f64,
}

impl CouplingState {
    /// Returns whether the complexes are balanced
    pub fn is_balanced(&self) -> bool {
        let max = self
            .mind_activation
            .max(self.body_activation)
            .max(self.spirit_activation);
        let min = self
            .mind_activation
            .min(self.body_activation)
            .min(self.spirit_activation);
        (max - min) < 0.3
    }

    /// Returns the dominant complex
    pub fn dominant_complex(&self) -> Option<ComplexAffiliation> {
        if self.mind_activation > self.body_activation
            && self.mind_activation > self.spirit_activation
        {
            Some(ComplexAffiliation::Mind)
        } else if self.body_activation > self.mind_activation
            && self.body_activation > self.spirit_activation
        {
            Some(ComplexAffiliation::Body)
        } else if self.spirit_activation > self.mind_activation
            && self.spirit_activation > self.body_activation
        {
            Some(ComplexAffiliation::Spirit)
        } else {
            None // All equal or all zero
        }
    }
}

/// Vibrational State reference for energy center activation
///
/// This is a simplified representation used by EnergyRayCenterSystem
#[derive(Debug, Clone)]
pub struct VibrationalStateRef {
    /// Overall vibrational level (0.0 to 1.0)
    pub overall_level: f64,
    /// Mind complex vibration (0.0 to 1.0)
    pub mind_vibration: f64,
    /// Body complex vibration (0.0 to 1.0)
    pub body_vibration: f64,
    /// Spirit complex vibration (0.0 to 1.0)
    pub spirit_vibration: f64,
    /// Current density level (1-7)
    pub density_level: u8,
}

impl EnergyRayCenterSystem {
    /// Activate energy centers via Free Will (Phase 6)
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 5.2
    /// "The Entity can make a CHOICE to activate higher potentiated centers"
    pub fn activate_via_free_will(&mut self, vibrational_state: &VibrationalStateRef) {
        // Convert vibrational state to simplified reference
        let vib_ref = VibrationalStateRef {
            overall_level: vibrational_state.overall_level,
            mind_vibration: vibrational_state.mind_vibration,
            body_vibration: vibrational_state.body_vibration,
            spirit_vibration: vibrational_state.spirit_vibration,
            density_level: vibrational_state.density_level,
        };

        // Activate centers based on vibrational state and density level
        match vib_ref.density_level {
            1 => {
                // First density: Red ray can activate
                self.activate_center(RayCenter::Red, vib_ref.body_vibration);
            }
            2 => {
                // Second density: Red and Orange rays can activate
                self.activate_center(RayCenter::Red, vib_ref.body_vibration);
                self.activate_center(RayCenter::Orange, vib_ref.body_vibration);
            }
            3 => {
                // Third density: Red, Orange, Yellow rays can activate
                self.activate_center(RayCenter::Red, vib_ref.body_vibration);
                self.activate_center(RayCenter::Orange, vib_ref.body_vibration);
                self.activate_center(RayCenter::Yellow, vib_ref.mind_vibration);
            }
            4 => {
                // Fourth density: Green ray can activate (heart center)
                self.activate_center(RayCenter::Red, vib_ref.body_vibration);
                self.activate_center(RayCenter::Orange, vib_ref.body_vibration);
                self.activate_center(RayCenter::Yellow, vib_ref.mind_vibration);
                self.activate_center(RayCenter::Green, vib_ref.overall_level);
            }
            5 => {
                // Fifth density: Blue ray can activate
                self.activate_center(RayCenter::Red, vib_ref.body_vibration);
                self.activate_center(RayCenter::Orange, vib_ref.body_vibration);
                self.activate_center(RayCenter::Yellow, vib_ref.mind_vibration);
                self.activate_center(RayCenter::Green, vib_ref.overall_level);
                self.activate_center(RayCenter::Blue, vib_ref.mind_vibration);
            }
            6 => {
                // Sixth density: Indigo ray can activate
                self.activate_center(RayCenter::Red, vib_ref.body_vibration);
                self.activate_center(RayCenter::Orange, vib_ref.body_vibration);
                self.activate_center(RayCenter::Yellow, vib_ref.mind_vibration);
                self.activate_center(RayCenter::Green, vib_ref.overall_level);
                self.activate_center(RayCenter::Blue, vib_ref.mind_vibration);
                self.activate_center(RayCenter::Indigo, vib_ref.spirit_vibration);
            }
            7 => {
                // Seventh density: Violet ray can activate
                self.activate_center(RayCenter::Red, vib_ref.body_vibration);
                self.activate_center(RayCenter::Orange, vib_ref.body_vibration);
                self.activate_center(RayCenter::Yellow, vib_ref.mind_vibration);
                self.activate_center(RayCenter::Green, vib_ref.overall_level);
                self.activate_center(RayCenter::Blue, vib_ref.mind_vibration);
                self.activate_center(RayCenter::Indigo, vib_ref.spirit_vibration);
                self.activate_center(RayCenter::Violet, vib_ref.spirit_vibration);
            }
            _ => {}
        }
    }

    /// Activate a specific energy center
    fn activate_center(&mut self, center: RayCenter, activation_factor: f64) {
        if let Some(state) = self.ray_centers.get_mut(&center) {
            // Activate based on vibrational factor
            let activation_increment = activation_factor * 0.05;
            state.activation_level = (state.activation_level + activation_increment).min(1.0);

            // Update energy level based on activation
            state.energy_level = state.energy_capacity * state.activation_level;

            // Update rotational speed
            state.rotational_speed = state.activation_level;

            // Balance the center
            state.balance();
        }
    }

    /// Absorb integrated flow (Green Ray activation)
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 5.2
    /// "The 'up-pouring' meets the 'in-pouring'"
    pub fn absorb_integrated_flow(
        &mut self,
        experience: crate::entity_state::Experience,
        intelligent_energy: crate::entity_state::IntelligentEnergy,
    ) {
        if let Some(green_state) = self.ray_centers.get_mut(&RayCenter::Green) {
            // Combine experience and intelligent energy
            let integrated_energy = experience.integration_level * intelligent_energy.intensity;

            // Receive the integrated energy
            green_state.receive_energy(integrated_energy);

            // Update sub-colors
            let sub_color_share = integrated_energy / 7.0;
            for sub_color in green_state.sub_colors.iter_mut() {
                sub_color.activate(sub_color_share);
                sub_color.balance_sub_color();
            }

            // Balance the green ray
            green_state.balance();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_center_names() {
        assert_eq!(RayCenter::Red.name(), "Red Ray");
        assert_eq!(RayCenter::Violet.name(), "Violet Ray");
    }

    #[test]
    fn test_ray_center_bodies() {
        assert_eq!(RayCenter::Red.associated_body(), EnergyRayBody::Chemical);
        assert_eq!(RayCenter::Violet.associated_body(), EnergyRayBody::Buddha);
    }

    #[test]
    fn test_primary_rays() {
        assert!(RayCenter::Red.is_primary());
        assert!(RayCenter::Yellow.is_primary());
        assert!(RayCenter::Blue.is_primary());
        assert!(!RayCenter::Orange.is_primary());
        assert!(!RayCenter::Green.is_primary());
        assert!(!RayCenter::Indigo.is_primary());
        assert!(!RayCenter::Violet.is_primary());
    }

    #[test]
    fn test_ray_center_state_creation() {
        let state = RayCenterState::new(RayCenter::Red);
        assert_eq!(state.ray_center, RayCenter::Red);
        assert_eq!(state.activation_level, 0.0);
        assert_eq!(state.energy_level, 0.0);
    }

    #[test]
    fn test_receive_energy() {
        let mut state = RayCenterState::new(RayCenter::Red);
        state.receive_energy(50.0);
        assert_eq!(state.energy_level, 50.0);
        assert_eq!(state.activation_level, 0.5);
    }

    #[test]
    fn test_provide_energy() {
        let mut state = RayCenterState::new(RayCenter::Red);
        state.receive_energy(50.0);
        let provided = state.provide_energy(30.0);
        assert_eq!(provided, 30.0);
        assert_eq!(state.energy_level, 20.0);
    }

    #[test]
    fn test_blockage() {
        let mut state = RayCenterState::new(RayCenter::Red);
        state.add_blockage(0.5);
        assert_eq!(state.blockage_level, 0.5);
        state.receive_energy(100.0);
        // Only 50% should be received due to blockage
        assert_eq!(state.energy_level, 50.0);
    }

    #[test]
    fn test_energy_transformation() {
        let mut transformation = EnergyTransformation::new(EnergyType::LoveLight, 100.0);
        let transformed = transformation.transform_love_light_to_light_love();
        assert_eq!(transformed, 95.0);
        assert_eq!(transformation.energy_type, EnergyType::LightLove);
        assert!(transformation.is_complete());
    }

    #[test]
    fn test_progressive_solidification_involution() {
        let mut solidification = ProgressiveSolidification::new_involution(100.0);
        assert_eq!(solidification.current_ray_center, RayCenter::Violet);

        assert_eq!(solidification.advance(), Some(RayCenter::Indigo));
        assert_eq!(solidification.current_ray_center, RayCenter::Indigo);

        assert_eq!(solidification.advance(), Some(RayCenter::Blue));
        assert_eq!(solidification.current_ray_center, RayCenter::Blue);

        assert_eq!(solidification.advance(), Some(RayCenter::Green));
        assert_eq!(solidification.advance(), Some(RayCenter::Yellow));
        assert_eq!(solidification.advance(), Some(RayCenter::Orange));
        assert_eq!(solidification.advance(), Some(RayCenter::Red));

        assert_eq!(solidification.advance(), None);
        assert!(solidification.is_complete());
    }

    #[test]
    fn test_progressive_solidification_evolution() {
        let mut solidification = ProgressiveSolidification::new_evolution(100.0);
        assert_eq!(solidification.current_ray_center, RayCenter::Red);

        assert_eq!(solidification.advance(), Some(RayCenter::Orange));
        assert_eq!(solidification.current_ray_center, RayCenter::Orange);

        assert_eq!(solidification.advance(), Some(RayCenter::Yellow));
        assert_eq!(solidification.advance(), Some(RayCenter::Green));
        assert_eq!(solidification.advance(), Some(RayCenter::Blue));
        assert_eq!(solidification.advance(), Some(RayCenter::Indigo));
        assert_eq!(solidification.advance(), Some(RayCenter::Violet));

        assert_eq!(solidification.advance(), None);
        assert!(solidification.is_complete());
    }

    #[test]
    fn test_energy_ray_center_system_creation() {
        let entity_id: usize = 1;
        let system = EnergyRayCenterSystem::new(entity_id);

        assert_eq!(system.entity_id, entity_id);
        assert_eq!(system.ray_centers.len(), 7);
        assert_eq!(system.total_energy_capacity, 700.0);
        assert_eq!(system.get_total_energy_level(), 0.0);
    }

    #[test]
    fn test_receive_love_light() {
        let entity_id: usize = 1;
        let mut system = EnergyRayCenterSystem::new(entity_id);

        system.receive_love_light(100.0);

        assert!(system.get_total_energy_level() > 0.0);
        assert!(system.energy_transformation.is_some());
        assert!(system.progressive_solidification.is_some());
    }

    #[test]
    fn test_provide_energy_for_processing() {
        let entity_id: usize = 1;
        let mut system = EnergyRayCenterSystem::new(entity_id);

        system.receive_love_light(100.0);
        let provided = system.provide_energy_for_processing(50.0);

        assert_eq!(provided, 50.0);
        assert!(system.get_total_energy_level() < 95.0); // 100 - 5% transformation loss
    }

    #[test]
    fn test_evolve() {
        let entity_id: usize = 1;
        let mut system = EnergyRayCenterSystem::new(entity_id);

        let initial_capacity = system.total_energy_capacity;
        system.evolve(10.0);

        assert!(system.total_energy_capacity > initial_capacity);
    }

    #[test]
    fn test_ray_center_involution_order() {
        let order = RayCenter::involution_order();
        assert_eq!(order[0], RayCenter::Violet);
        assert_eq!(order[6], RayCenter::Red);
    }

    #[test]
    fn test_ray_center_evolution_order() {
        let order = RayCenter::evolution_order();
        assert_eq!(order[0], RayCenter::Red);
        assert_eq!(order[6], RayCenter::Violet);
    }

    #[test]
    fn test_energy_ray_body_solidification_level() {
        assert_eq!(EnergyRayBody::Chemical.solidification_level(), 1);
        assert_eq!(EnergyRayBody::Buddha.solidification_level(), 7);
    }

    #[test]
    fn test_is_ready_for_harvest() {
        let entity_id: usize = 1;
        let mut system = EnergyRayCenterSystem::new(entity_id);

        // Not ready initially
        assert!(!system.is_ready_for_harvest());

        // Activate violet ray and balance system
        if let Some(violet_state) = system.ray_centers.get_mut(&RayCenter::Violet) {
            violet_state.activation_level = 0.9;
            violet_state.balance_level = 0.8;
        }

        for state in system.ray_centers.values_mut() {
            state.balance_level = 0.8;
        }

        system.update_overall_balance();
        assert!(system.is_ready_for_harvest());
    }

    #[test]
    fn test_sub_color_creation() {
        let sub_color = SubColor::new(7, ComplexAffiliation::Mind); // Added archetype_index=7 (A8)
        assert_eq!(sub_color.activation, 0.0);
        assert_eq!(sub_color.balance, 0.5);
        assert_eq!(sub_color.complex_affiliation, ComplexAffiliation::Mind);
    }

    #[test]
    fn test_sub_color_activation() {
        let mut sub_color = SubColor::new(7, ComplexAffiliation::Body); // Added archetype_index=7 (A8)
        sub_color.activate(0.5);
        assert_eq!(sub_color.activation, 0.5);
        sub_color.activate(0.6);
        assert_eq!(sub_color.activation, 1.0); // Should cap at 1.0
    }

    #[test]
    fn test_sub_color_balance() {
        let mut sub_color = SubColor::new(7, ComplexAffiliation::Spirit); // Added archetype_index=7 (A8)
        sub_color.activation = 0.8;
        sub_color.balance_sub_color();
        assert!(sub_color.balance > 0.5);
    }

    #[test]
    fn test_ray_center_has_sub_colors() {
        let state = RayCenterState::new(RayCenter::Red);
        assert_eq!(state.sub_colors.len(), 7);
    }

    #[test]
    fn test_ray_center_sub_colors_distribution() {
        let red_state = RayCenterState::new(RayCenter::Red);
        let violet_state = RayCenterState::new(RayCenter::Violet);

        // Red ray should be mostly Body complex
        let red_body_count = red_state
            .sub_colors
            .iter()
            .filter(|sc| sc.complex_affiliation == ComplexAffiliation::Body)
            .count();
        assert_eq!(red_body_count, 7);

        // Violet ray should be mostly Spirit complex
        let violet_spirit_count = violet_state
            .sub_colors
            .iter()
            .filter(|sc| sc.complex_affiliation == ComplexAffiliation::Spirit)
            .count();
        assert_eq!(violet_spirit_count, 7);
    }

    #[test]
    fn test_receive_energy_distributes_to_sub_colors() {
        let mut state = RayCenterState::new(RayCenter::Red);
        state.receive_energy(70.0);

        // Check that sub-colors received energy
        let avg_activation = state.get_sub_color_average_activation();
        assert!(avg_activation > 0.0);
    }

    #[test]
    fn test_sub_color_average_activation() {
        let mut state = RayCenterState::new(RayCenter::Green);
        state.receive_energy(70.0);

        let avg = state.get_sub_color_average_activation();
        assert!(avg > 0.0 && avg <= 1.0);
    }

    #[test]
    fn test_sub_color_average_balance() {
        let state = RayCenterState::new(RayCenter::Blue);
        let avg = state.get_sub_color_average_balance();
        assert!(avg >= 0.0 && avg <= 1.0);
    }

    #[test]
    fn test_are_sub_colors_balanced() {
        let mut state = RayCenterState::new(RayCenter::Yellow);
        // Initially unbalanced (all sub-colors at 0.0)
        assert!(state.are_sub_colors_balanced());

        // Activate some sub-colors unevenly
        state.sub_colors[0].activation = 0.8;
        state.sub_colors[1].activation = 0.8;
        state.sub_colors[2].activation = 0.1;
        state.sub_colors[3].activation = 0.1;
        state.sub_colors[4].activation = 0.1;
        state.sub_colors[5].activation = 0.1;
        state.sub_colors[6].activation = 0.1;

        // Should be unbalanced now
        assert!(!state.are_sub_colors_balanced());
    }

    #[test]
    fn test_energy_flow_creation() {
        let flow = EnergyFlow::new(1.0, FlowDirection::UpPouring, RayCenter::Red);
        assert_eq!(flow.intensity, 1.0);
        assert_eq!(flow.direction, FlowDirection::UpPouring);
        assert_eq!(flow.ray_color, RayCenter::Red);
    }

    #[test]
    fn test_energy_flow_merge() {
        let flow1 = EnergyFlow::new(0.5, FlowDirection::UpPouring, RayCenter::Red);
        let flow2 = EnergyFlow::new(0.3, FlowDirection::InPouring, RayCenter::Violet);
        let merged = flow1.merge(flow2);

        assert_eq!(merged.intensity, 0.8);
        assert_eq!(merged.direction, FlowDirection::UpPouring); // Dominant direction
        assert_eq!(merged.ray_color, RayCenter::Red); // Dominant ray
    }

    #[test]
    fn test_integrate_flows_open_valve() {
        let entity_id: usize = 1;
        let mut system = EnergyRayCenterSystem::new(entity_id);

        let up_pouring = EnergyFlow::new(1.0, FlowDirection::UpPouring, RayCenter::Red);
        let in_pouring = EnergyFlow::new(0.8, FlowDirection::InPouring, RayCenter::Violet);

        let result =
            system.integrate_flows(RayCenter::Green, up_pouring, in_pouring, ValveState::Open);

        match result {
            IntegrationResult::Full(flow) => {
                assert_eq!(flow.intensity, 1.8);
            }
            _ => panic!("Expected Full integration result"),
        }
    }

    #[test]
    fn test_integrate_flows_restricted_valve() {
        let entity_id: usize = 1;
        let mut system = EnergyRayCenterSystem::new(entity_id);

        let up_pouring = EnergyFlow::new(1.0, FlowDirection::UpPouring, RayCenter::Red);
        let in_pouring = EnergyFlow::new(0.8, FlowDirection::InPouring, RayCenter::Violet);

        let result = system.integrate_flows(
            RayCenter::Green,
            up_pouring,
            in_pouring,
            ValveState::Restricted,
        );

        match result {
            IntegrationResult::Partial(flow) => {
                assert_eq!(flow.intensity, 1.4); // 1.0 + (0.8 * 0.5)
            }
            _ => panic!("Expected Partial integration result"),
        }
    }

    #[test]
    fn test_integrate_flows_closed_valve() {
        let entity_id: usize = 1;
        let mut system = EnergyRayCenterSystem::new(entity_id);

        let up_pouring = EnergyFlow::new(1.0, FlowDirection::UpPouring, RayCenter::Red);
        let in_pouring = EnergyFlow::new(0.8, FlowDirection::InPouring, RayCenter::Violet);

        let result =
            system.integrate_flows(RayCenter::Green, up_pouring, in_pouring, ValveState::Closed);

        match result {
            IntegrationResult::Blocked(flow) => {
                assert_eq!(flow.intensity, 1.0);
            }
            _ => panic!("Expected Blocked integration result"),
        }
    }

    #[test]
    fn test_process_energy_flow_evolution() {
        let entity_id: usize = 1;
        let mut system = EnergyRayCenterSystem::new(entity_id);

        let flow = EnergyFlow::new(1.0, FlowDirection::UpPouring, RayCenter::Red);
        let results = system.process_energy_flow(flow, SolidificationDirection::Evolution);

        // Should process through all 7 centers
        assert_eq!(results.len(), 7);
    }

    #[test]
    fn test_process_energy_flow_involution() {
        let entity_id: usize = 1;
        let mut system = EnergyRayCenterSystem::new(entity_id);

        let flow = EnergyFlow::new(1.0, FlowDirection::InPouring, RayCenter::Violet);
        let results = system.process_energy_flow(flow, SolidificationDirection::Involution);

        // Should process through all 7 centers
        assert_eq!(results.len(), 7);
    }

    #[test]
    fn test_determine_valve_state() {
        let entity_id: usize = 1;
        let mut system = EnergyRayCenterSystem::new(entity_id);

        // Low balance -> Closed valve
        if let Some(state) = system.ray_centers.get_mut(&RayCenter::Green) {
            state.balance_level = 0.3;
        }
        let valve = system.determine_valve_state(
            RayCenter::Green,
            EnergyFlow::new(1.0, FlowDirection::UpPouring, RayCenter::Red),
        );
        assert_eq!(valve, ValveState::Closed);

        // Medium balance -> Restricted valve
        if let Some(state) = system.ray_centers.get_mut(&RayCenter::Green) {
            state.balance_level = 0.5;
        }
        let valve = system.determine_valve_state(
            RayCenter::Green,
            EnergyFlow::new(1.0, FlowDirection::UpPouring, RayCenter::Red),
        );
        assert_eq!(valve, ValveState::Restricted);

        // High balance -> Open valve
        if let Some(state) = system.ray_centers.get_mut(&RayCenter::Green) {
            state.balance_level = 0.8;
        }
        let valve = system.determine_valve_state(
            RayCenter::Green,
            EnergyFlow::new(1.0, FlowDirection::UpPouring, RayCenter::Red),
        );
        assert_eq!(valve, ValveState::Open);
    }

    #[test]
    fn test_get_coupling_state() {
        let entity_id: usize = 1;
        let mut system = EnergyRayCenterSystem::new(entity_id);

        let coupling = system.get_coupling_state();
        assert!(coupling.overall_coupling >= 0.0 && coupling.overall_coupling <= 1.0);
    }

    #[test]
    fn test_coupling_state_is_balanced() {
        let entity_id: usize = 1;
        let mut system = EnergyRayCenterSystem::new(entity_id);

        let coupling = system.get_coupling_state();
        // Initially should be balanced (all zeros)
        assert!(coupling.is_balanced());
    }

    #[test]
    fn test_coupling_state_dominant_complex() {
        let entity_id: usize = 1;
        let mut system = EnergyRayCenterSystem::new(entity_id);

        // Activate Body complex sub-colors
        for state in system.ray_centers.values_mut() {
            for sub_color in state.sub_colors.iter_mut() {
                if sub_color.complex_affiliation == ComplexAffiliation::Body {
                    sub_color.activation = 0.8;
                }
            }
        }

        let coupling = system.get_coupling_state();
        assert_eq!(coupling.dominant_complex(), Some(ComplexAffiliation::Body));
    }

    // ============================================================================
    // Phase 2.3: Red Ray as Densest Packing Tests
    // ============================================================================

    /// Test that Red Ray is NOT low or simple
    ///
    /// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.7
    /// "The Red Ray is NOT 'low' or 'simple.' It is the Densest Packing of the Violet Ray."
    #[test]
    fn test_red_ray_not_low_or_simple() {
        let red = RayCenter::Red;
        assert!(
            !red.is_low_or_simple(),
            "Red Ray should NOT be low or simple"
        );

        // Other rays are simpler relative to Red
        assert!(RayCenter::Orange.is_low_or_simple());
        assert!(RayCenter::Yellow.is_low_or_simple());
    }

    /// Test that Red Ray contains packed rays
    #[test]
    fn test_red_ray_contains_packed_rays() {
        let red = RayCenter::Red;
        assert!(red.can_contain_packed_rays());
        assert_eq!(red.contained_ray_count(), 6);

        let contained = red.contained_rays();
        assert_eq!(contained.len(), 6);
        assert!(contained.contains(&RayCenter::Orange));
        assert!(contained.contains(&RayCenter::Yellow));
        assert!(contained.contains(&RayCenter::Green));
        assert!(contained.contains(&RayCenter::Blue));
        assert!(contained.contains(&RayCenter::Indigo));
        assert!(contained.contains(&RayCenter::Violet));
    }

    /// Test that Red Ray is the densest packing
    #[test]
    fn test_red_ray_as_densest_packing() {
        let red = RayCenter::Red;
        assert!(red.is_densest_packing());

        // Other rays are not densest packing
        assert!(!RayCenter::Orange.is_densest_packing());
        assert!(!RayCenter::Violet.is_densest_packing());
    }

    /// Test that Red Ray has high compression level
    #[test]
    fn test_red_ray_compression_level() {
        let red = RayCenter::Red;
        let compression = red.compression_level().unwrap();
        assert!(
            compression > 0.95,
            "Red Ray should have high compression ratio"
        );

        // Other rays have no compression
        assert!(RayCenter::Orange.compression_level().is_none());
    }

    /// Test that Red Ray has high density of packed potential
    #[test]
    fn test_red_ray_density_of_packed_potential() {
        let red = RayCenter::Red;
        assert_eq!(red.density_of_packed_potential(), 6.0);

        // Other rays have no packed potential density
        assert_eq!(RayCenter::Orange.density_of_packed_potential(), 0.0);
    }

    /// Test that Red Ray has philosophical depth
    #[test]
    fn test_red_ray_philosophical_depth() {
        let red = RayCenter::Red;
        let description = red.philosophical_description();

        assert!(description.contains("Densest Packing"));
        assert!(description.contains("Violet Ray"));
        assert!(description.contains("Bottom of the Wave"));
        assert!(description.contains("folded within"));
    }

    /// Test PackedRay structure
    #[test]
    fn test_packed_ray_creation() {
        let packed = PackedRay::new(RayCenter::Violet, 0.99);
        assert_eq!(packed.ray_type, RayCenter::Violet);
        assert_eq!(packed.compression_ratio, 0.99);
        assert!(!packed.is_accessible);
        assert!(packed.contained_potential > 0.0);
    }

    /// Test PackedRay unpacking
    #[test]
    fn test_packed_ray_unpacking() {
        let mut packed = PackedRay::new(RayCenter::Violet, 0.99);
        let initial_potential = packed.contained_potential;

        // Cannot unpack if not accessible
        let unpacked = packed.unpack();
        assert_eq!(unpacked, 0.0);
        assert_eq!(packed.contained_potential, initial_potential);

        // Make accessible and unpack
        packed.make_accessible();
        let unpacked = packed.unpack();
        assert_eq!(unpacked, initial_potential);
        assert_eq!(packed.contained_potential, 0.0);
    }

    /// Test PackedRay is fully compressed
    #[test]
    fn test_packed_ray_fully_compressed() {
        let packed = PackedRay::new(RayCenter::Violet, 0.99);
        assert!(packed.is_fully_compressed());

        let less_compressed = PackedRay::new(RayCenter::Orange, 0.50);
        assert!(!less_compressed.is_fully_compressed());
    }

    /// Test PackedRays structure
    #[test]
    fn test_packed_rays_creation() {
        let packed_rays = PackedRays::new();
        assert_eq!(packed_rays.all_rays().len(), 6);

        // Verify all rays are present
        let ray_types: Vec<RayCenter> = packed_rays
            .all_rays()
            .iter()
            .map(|ray| ray.ray_type)
            .collect();
        assert!(ray_types.contains(&RayCenter::Orange));
        assert!(ray_types.contains(&RayCenter::Yellow));
        assert!(ray_types.contains(&RayCenter::Green));
        assert!(ray_types.contains(&RayCenter::Blue));
        assert!(ray_types.contains(&RayCenter::Indigo));
        assert!(ray_types.contains(&RayCenter::Violet));
    }

    /// Test PackedRays compression ratios increase with ray level
    #[test]
    fn test_packed_rays_compression_ratios() {
        let packed_rays = PackedRays::new();

        // Violet should have highest compression
        assert!(
            packed_rays.violet_ray.compression_ratio > packed_rays.indigo_ray.compression_ratio
        );
        assert!(packed_rays.indigo_ray.compression_ratio > packed_rays.blue_ray.compression_ratio);
        assert!(packed_rays.blue_ray.compression_ratio > packed_rays.green_ray.compression_ratio);
        assert!(packed_rays.green_ray.compression_ratio > packed_rays.yellow_ray.compression_ratio);
        assert!(
            packed_rays.yellow_ray.compression_ratio > packed_rays.orange_ray.compression_ratio
        );
    }

    /// Test PackedRays total contained potential
    #[test]
    fn test_packed_rays_total_contained_potential() {
        let packed_rays = PackedRays::new();
        let total = packed_rays.total_contained_potential();
        assert!(total > 0.0);

        // Total should be sum of all ray potentials
        let sum: f64 = packed_rays
            .all_rays()
            .iter()
            .map(|ray| ray.contained_potential)
            .sum();
        assert_eq!(total, sum);
    }

    /// Test PackedRays average compression ratio
    #[test]
    fn test_packed_rays_average_compression_ratio() {
        let packed_rays = PackedRays::new();
        let avg = packed_rays.average_compression_ratio();
        assert!(avg > 0.0 && avg < 1.0);
    }

    /// Test PackedRays make ray accessible
    #[test]
    fn test_packed_rays_make_ray_accessible() {
        let mut packed_rays = PackedRays::new();

        // Initially, no rays are accessible
        assert!(!packed_rays.orange_ray.is_accessible);
        assert!(!packed_rays.violet_ray.is_accessible);

        // Make Violet accessible
        assert!(packed_rays.make_ray_accessible(RayCenter::Violet));
        assert!(packed_rays.violet_ray.is_accessible);

        // Cannot make Red accessible
        assert!(!packed_rays.make_ray_accessible(RayCenter::Red));
    }

    /// Test RayCenterType Red variant
    #[test]
    fn test_ray_center_type_red_creation() {
        let red_type = RayCenterType::new_red();

        assert!(red_type.is_red_with_packed_rays());
        assert!(!red_type.has_begun_unpacking());
        assert!(red_type.compression_ratio() > 0.95);
        assert!(red_type.contained_potential() > 0.0);
    }

    /// Test RayCenterType unpacking
    #[test]
    fn test_ray_center_type_unpacking() {
        let mut red_type = RayCenterType::new_red();

        // Initially, no rays are accessible
        assert_eq!(red_type.accessible_ray_count(), 0);

        // Make some rays accessible
        red_type.make_ray_accessible(RayCenter::Orange);
        red_type.make_ray_accessible(RayCenter::Violet);

        assert_eq!(red_type.accessible_ray_count(), 2);

        // Unpack
        let unpacked = red_type.unpack();
        assert!(red_type.has_begun_unpacking());
        assert_eq!(unpacked.len(), 2);

        // Verify unpacked rays
        let unpacked_types: Vec<RayCenter> = unpacked.iter().map(|(ray, _)| *ray).collect();
        assert!(unpacked_types.contains(&RayCenter::Orange));
        assert!(unpacked_types.contains(&RayCenter::Violet));
    }

    /// Test RayCenterType unpacking returns potential
    #[test]
    fn test_ray_center_type_unpacking_returns_potential() {
        let mut red_type = RayCenterType::new_red();
        let initial_potential = red_type.contained_potential();

        // Make all rays accessible
        red_type.make_ray_accessible(RayCenter::Orange);
        red_type.make_ray_accessible(RayCenter::Yellow);
        red_type.make_ray_accessible(RayCenter::Green);
        red_type.make_ray_accessible(RayCenter::Blue);
        red_type.make_ray_accessible(RayCenter::Indigo);
        red_type.make_ray_accessible(RayCenter::Violet);

        // Unpack all
        let unpacked = red_type.unpack();
        let total_unpacked: f64 = unpacked.iter().map(|(_, potential)| *potential).sum();

        // Should have unpacked most of the potential
        assert!(total_unpacked > 0.0);
        assert!(total_unpacked <= initial_potential);
    }

    /// Test RayCenterType other variants cannot unpack
    #[test]
    fn test_ray_center_type_other_rays_cannot_unpack() {
        let mut orange_type = RayCenterType::Orange;
        let unpacked = orange_type.unpack();
        assert_eq!(unpacked.len(), 0);

        let mut violet_type = RayCenterType::Violet;
        let unpacked = violet_type.unpack();
        assert_eq!(unpacked.len(), 0);
    }

    /// Test RayCenterType description
    #[test]
    fn test_ray_center_type_description() {
        let red_type = RayCenterType::new_red();
        let description = red_type.description();

        assert!(description.contains("Densest Packing"));
        assert!(description.contains("Violet Ray"));
        assert!(description.contains("folded within"));
    }

    /// Test RayCenterType contained potential for non-Red rays
    #[test]
    fn test_ray_center_type_contained_potential_non_red() {
        let orange_type = RayCenterType::Orange;
        assert_eq!(orange_type.contained_potential(), 0.0);

        let violet_type = RayCenterType::Violet;
        assert_eq!(violet_type.contained_potential(), 0.0);
    }

    /// Test RayCenterType compression ratio for non-Red rays
    #[test]
    fn test_ray_center_type_compression_ratio_non_red() {
        let orange_type = RayCenterType::Orange;
        assert_eq!(orange_type.compression_ratio(), 0.0);

        let violet_type = RayCenterType::Violet;
        assert_eq!(violet_type.compression_ratio(), 0.0);
    }

    /// Test RayCenterType accessible ray count for non-Red rays
    #[test]
    fn test_ray_center_type_accessible_ray_count_non_red() {
        let orange_type = RayCenterType::Orange;
        assert_eq!(orange_type.accessible_ray_count(), 0);

        let violet_type = RayCenterType::Violet;
        assert_eq!(violet_type.accessible_ray_count(), 0);
    }

    /// Test comprehensive: Red Ray as Densest Packing concept
    #[test]
    fn test_red_ray_as_densest_packing_comprehensive() {
        // Create Red Ray type
        let mut red_type = RayCenterType::new_red();

        // Verify Red is densest packing
        assert!(red_type.is_red_with_packed_rays());
        assert!(red_type.compression_ratio() > 0.95);
        assert!(red_type.contained_potential() > 0.0);

        // Verify all higher rays are contained
        let red = RayCenter::Red;
        assert_eq!(red.contained_ray_count(), 6);
        assert!(red.is_densest_packing());
        assert!(!red.is_low_or_simple());

        // Make rays accessible and unpack
        red_type.make_ray_accessible(RayCenter::Orange);
        red_type.make_ray_accessible(RayCenter::Yellow);
        red_type.make_ray_accessible(RayCenter::Green);
        red_type.make_ray_accessible(RayCenter::Blue);
        red_type.make_ray_accessible(RayCenter::Indigo);
        red_type.make_ray_accessible(RayCenter::Violet);

        let unpacked = red_type.unpack();
        assert_eq!(unpacked.len(), 6);
        assert!(red_type.has_begun_unpacking());

        // Verify philosophical depth
        let description = red.philosophical_description();
        assert!(description.contains("Densest Packing"));
        assert!(description.contains("Violet Ray"));
        assert!(description.contains("Bottom of the Wave"));
    }

    // ===== PHASE 5: Energy Centers as Interfaces - Validation Tests =====

    /// Test SubColor includes archetype_index
    ///
    /// PHASE 5 Validation Criterion:
    /// "Sub-colors (7 per center) are implemented"
    /// "Each sub-color interfaces to a specific archetype"
    #[test]
    fn test_phase5_subcolor_has_archetype_index() {
        let sub_color = SubColor::new(7, ComplexAffiliation::Body);

        // Verify archetype_index is set
        assert_eq!(sub_color.archetype_index, 7);

        // Verify other fields are initialized
        assert_eq!(sub_color.activation, 0.0);
        assert_eq!(sub_color.balance, 0.5);
        assert_eq!(sub_color.complex_affiliation, ComplexAffiliation::Body);
    }

    /// Test SubColor archetype_index wraps to valid range
    ///
    /// PHASE 5 Validation Criterion:
    /// "Each sub-color interfaces to a specific archetype from the 22-Archetype structure"
    #[test]
    fn test_phase5_subcolor_archetype_index_wraps() {
        // Test that archetype_index wraps to valid range (0-21)
        let sub_color = SubColor::new(25, ComplexAffiliation::Spirit);
        assert_eq!(sub_color.archetype_index, 3); // 25 % 22 = 3

        let sub_color2 = SubColor::new(22, ComplexAffiliation::Mind);
        assert_eq!(sub_color2.archetype_index, 0); // 22 % 22 = 0
    }

    /// Test CenterCoupling structure
    ///
    /// PHASE 5 Validation Criterion:
    /// "Energy Centers are interfaces, not containers"
    /// "How Mind, Body, Spirit aspects are interacting at this level"
    #[test]
    fn test_phase5_center_coupling_structure() {
        let coupling = CenterCoupling::new();

        // Verify default values
        assert_eq!(coupling.mind_aspect, 0.5);
        assert_eq!(coupling.body_aspect, 0.5);
        assert_eq!(coupling.spirit_aspect, 0.5);

        // Verify overall balance
        assert_eq!(coupling.overall_balance(), 0.5);
    }

    /// Test CenterCoupling with custom values
    #[test]
    fn test_phase5_center_coupling_with_values() {
        let coupling = CenterCoupling::with_values(0.8, 0.6, 0.9);

        assert_eq!(coupling.mind_aspect, 0.8);
        assert_eq!(coupling.body_aspect, 0.6);
        assert_eq!(coupling.spirit_aspect, 0.9);

        // Verify clamping to valid range
        let clamped = CenterCoupling::with_values(1.5, -0.5, 0.5);
        assert_eq!(clamped.mind_aspect, 1.0);
        assert_eq!(clamped.body_aspect, 0.0);
    }

    /// Test CenterCoupling update
    #[test]
    fn test_phase5_center_coupling_update() {
        let mut coupling = CenterCoupling::new();
        coupling.update_from_aspects(0.9, 0.7, 0.8);

        assert_eq!(coupling.mind_aspect, 0.9);
        assert_eq!(coupling.body_aspect, 0.7);
        assert_eq!(coupling.spirit_aspect, 0.8);
    }

    /// Test EnergyCenter as interface (not container)
    ///
    /// PHASE 5 Validation Criterion:
    /// "Energy Centers are interfaces, not containers"
    /// "They don't 'contain' anything—they 'unlock' what's folded within"
    #[test]
    fn test_phase5_energy_center_as_interface() {
        let center = EnergyCenter::new(RayCenter::Green);

        // Verify center is initialized as interface
        assert_eq!(center.ray_center, RayCenter::Green);
        assert_eq!(center.activation, 0.0); // Initially not activated
        assert_eq!(center.balance, 0.5); // Initial balance

        // Verify sub-colors exist (internal differentiation)
        assert_eq!(center.sub_colors.len(), 7);

        // Verify coupling exists
        assert_eq!(center.coupling.overall_balance(), 0.5);

        // Verify archetype mapping exists
        assert!(!center.archetype_mapping.is_empty());
    }

    /// Test EnergyCenter sub-colors have archetype indices
    ///
    /// PHASE 5 Validation Criterion:
    /// "Each center has 7 sub-colors"
    /// "Each sub-color interfaces to a specific archetype"
    #[test]
    fn test_phase5_energy_center_subcolors_have_archetypes() {
        let center = EnergyCenter::new(RayCenter::Green);

        // Verify all sub-colors have archetype indices
        for sub_color in &center.sub_colors {
            assert!(sub_color.archetype_index < 22); // Valid archetype index
        }

        // Verify Green Ray has mixed archetypes (bridge)
        // Green should have archetypes from all complexes
        let archetypes: Vec<u8> = center
            .sub_colors
            .iter()
            .map(|sc| sc.archetype_index)
            .collect();
        assert!(archetypes.len() == 7);
    }

    /// Test EnergyCenter unlock mechanism with Free Will
    ///
    /// PHASE 5 Validation Criterion:
    /// "Unlock mechanism is implemented"
    /// "Free Will choice mechanism is implemented"
    /// "Spiral development is supported"
    #[test]
    fn test_phase5_energy_center_unlock() {
        let mut center = EnergyCenter::new(RayCenter::Blue);

        // Initial state
        assert_eq!(center.activation, 0.0);

        // Unlock with Free Will activation
        let unlock_amount = center.unlock(0.8);

        // Verify activation increased
        assert!(unlock_amount > 0.0);
        assert!(center.activation > 0.0);
        assert!(center.activation <= 1.0);

        // Verify balance also improved
        assert!(center.balance >= 0.5);

        // Verify sub-colors updated
        for sub_color in &center.sub_colors {
            assert!(sub_color.activation >= 0.0);
        }

        // Verify coupling updated
        assert!(center.coupling.mind_aspect >= 0.5);
    }

    /// Test EnergyCenter unlock with zero Free Will
    #[test]
    fn test_phase5_energy_center_unlock_zero_free_will() {
        let mut center = EnergyCenter::new(RayCenter::Indigo);

        let initial_activation = center.activation;

        // Unlock with zero Free Will
        let unlock_amount = center.unlock(0.0);

        // Verify no unlock occurred
        assert_eq!(unlock_amount, 0.0);
        assert_eq!(center.activation, initial_activation);
    }

    /// Test EnergyCenter unlock with full Free Will
    #[test]
    fn test_phase5_energy_center_unlock_full_free_will() {
        let mut center = EnergyCenter::new(RayCenter::Violet);

        // Unlock multiple times with full Free Will
        for _ in 0..15 {
            center.unlock(1.0);
        }

        // Verify activation capped at 1.0
        assert_eq!(center.activation, 1.0);
        assert!(center.is_fully_activated());
    }

    /// Test EnergyCenter balance_lower_centers
    ///
    /// PHASE 5 Validation Criterion:
    /// "Spiral development is supported"
    /// "Support choice to balance lower centers"
    #[test]
    fn test_phase5_energy_center_balance_lower_centers() {
        let mut center = EnergyCenter::new(RayCenter::Yellow);

        let initial_balance = center.balance;
        let initial_activation = center.activation;

        // Balance lower centers
        center.balance_lower_centers(0.8);

        // Verify balance increased
        assert!(center.balance > initial_balance);

        // Verify activation did NOT increase significantly
        // (balance_lower_centers should not significantly increase activation)
        assert!(center.activation < initial_activation + 0.1);
    }

    /// Test EnergyCenter integrate_flows (Green Ray)
    ///
    /// PHASE 5 Validation Criterion:
    /// "Green Ray specific flow integration"
    /// "Bridge between lower and higher centers"
    #[test]
    fn test_phase5_energy_center_integrate_flows() {
        let mut center = EnergyCenter::new(RayCenter::Green);

        let initial_activation = center.activation;
        let initial_balance = center.balance;

        // Integrate flows (up-pouring meets in-pouring)
        center.integrate_flows(0.7, 0.8);

        // Verify activation increased
        assert!(center.activation > initial_activation);

        // Verify balance improved
        assert!(center.balance >= initial_balance);

        // Verify coupling updated
        assert!(center.coupling.body_aspect > 0.5);
        assert!(center.coupling.spirit_aspect > 0.5);
    }

    /// Test EnergyCenter partial_integration (Restricted valve)
    ///
    /// PHASE 5 Validation Criterion:
    /// "Partial integration - some Spirit access, but limited"
    #[test]
    fn test_phase5_energy_center_partial_integration() {
        let mut center = EnergyCenter::new(RayCenter::Green);

        let initial_activation = center.activation;

        // Partial integration (Restricted valve)
        center.partial_integration(0.7, 0.8);

        // Verify activation increased (but less than full integration)
        assert!(center.activation > initial_activation);
        assert!(center.activation < 0.2); // Less than full integration

        // Verify Spirit aspect is limited
        // (partial_integration has spirit_factor = 0.5)
        assert!(center.coupling.spirit_aspect < 0.6);
    }

    /// Test EnergyCenter center-archetype mapping
    ///
    /// PHASE 5 Validation Criterion:
    /// "Center-archetype mapping is correct"
    /// Red (Center 1): Body Complex (A8)
    /// Orange (Center 2): Body Complex (A9)
    /// Yellow (Center 3): Mind Complex (A3)
    /// Green (Center 4): Bridge (All complexes)
    /// Blue (Center 5): Spirit Complex (A16)
    /// Indigo (Center 6): Spirit Complex (A17)
    /// Violet (Center 7): Spirit Complex (A21)
    #[test]
    fn test_phase5_energy_center_archetype_mapping() {
        // Red (Center 1): Body Complex (A8)
        let red = EnergyCenter::new(RayCenter::Red);
        assert!(red.archetype_mapping.iter().all(|&a| a == 7)); // A8

        // Orange (Center 2): Body Complex (A9)
        let orange = EnergyCenter::new(RayCenter::Orange);
        assert!(orange.archetype_mapping.iter().all(|&a| a == 8)); // A9

        // Yellow (Center 3): Mind Complex (A3)
        let yellow = EnergyCenter::new(RayCenter::Yellow);
        assert!(yellow.archetype_mapping.iter().all(|&a| a == 2)); // A3

        // Green (Center 4): Bridge (All complexes)
        let green = EnergyCenter::new(RayCenter::Green);
        assert!(green.archetype_mapping.contains(&2)); // A3
        assert!(green.archetype_mapping.contains(&7)); // A8
        assert!(green.archetype_mapping.contains(&8)); // A9
        assert!(green.archetype_mapping.contains(&15)); // A16
        assert!(green.archetype_mapping.contains(&16)); // A17
        assert!(green.archetype_mapping.contains(&20)); // A21
        assert!(green.archetype_mapping.contains(&21)); // A22

        // Blue (Center 5): Spirit Complex (A16)
        let blue = EnergyCenter::new(RayCenter::Blue);
        assert!(blue.archetype_mapping.iter().all(|&a| a == 15)); // A16

        // Indigo (Center 6): Spirit Complex (A17)
        let indigo = EnergyCenter::new(RayCenter::Indigo);
        assert!(indigo.archetype_mapping.iter().all(|&a| a == 16)); // A17

        // Violet (Center 7): Spirit Complex (A21)
        let violet = EnergyCenter::new(RayCenter::Violet);
        assert!(violet.archetype_mapping.iter().all(|&a| a == 20)); // A21
    }

    /// Test EnergyCenter spiral development (non-linear leaps)
    ///
    /// PHASE 5 Validation Criterion:
    /// "Spiral development is supported"
    /// "Support non-linear activation"
    /// "Support 'leaps' in consciousness"
    #[test]
    fn test_phase5_energy_center_spiral_development() {
        let mut center = EnergyCenter::new(RayCenter::Violet);

        // Initial state: not balanced
        assert!(!center.is_balanced());

        // Spiral development: unlock higher center even if not perfectly balanced
        // This represents a "leap" in consciousness
        let unlock1 = center.unlock(0.9); // High Free Will

        // Verify activation increased despite not being balanced
        assert!(unlock1 > 0.0);
        assert!(center.activation > 0.0);

        // Continue spiral development
        for _ in 0..5 {
            center.unlock(0.8);
        }

        // Verify significant activation achieved
        assert!(center.activation > 0.5);

        // Verify balance also improved over time
        assert!(center.balance > 0.5);
    }

    /// Test EnergyCenter is_fully_activated
    #[test]
    fn test_phase5_energy_center_is_fully_activated() {
        let mut center = EnergyCenter::new(RayCenter::Blue);

        // Initially not fully activated
        assert!(!center.is_fully_activated());

        // Unlock until fully activated
        for _ in 0..20 {
            center.unlock(1.0);
        }

        // Now fully activated
        assert!(center.is_fully_activated());
        assert_eq!(center.activation, 1.0);
    }

    /// Test EnergyCenter is_balanced
    #[test]
    fn test_phase5_energy_center_is_balanced() {
        let mut center = EnergyCenter::new(RayCenter::Green);

        // Initially not balanced (balance = 0.5)
        assert!(!center.is_balanced());

        // Balance until balanced
        for _ in 0..5 {
            center.balance_lower_centers(1.0);
        }

        // Now balanced
        assert!(center.is_balanced());
    }

    /// Test EnergyCenter sub_color_average_activation
    #[test]
    fn test_phase5_energy_center_sub_color_average_activation() {
        let mut center = EnergyCenter::new(RayCenter::Yellow);

        // Initial average should be 0.0
        assert_eq!(center.sub_color_average_activation(), 0.0);

        // Unlock some
        center.unlock(0.5);

        // Average should increase
        assert!(center.sub_color_average_activation() > 0.0);
    }

    /// Test EnergyCenter sub_color_average_balance
    #[test]
    fn test_phase5_energy_center_sub_color_average_balance() {
        let center = EnergyCenter::new(RayCenter::Blue);

        // Initial average should be 0.5
        assert_eq!(center.sub_color_average_balance(), 0.5);
    }

    /// Test comprehensive Phase 5 validation
    ///
    /// PHASE 5 Validation Criteria:
    /// - Energy Centers are interfaces, not containers
    /// - Sub-colors (7 per center) are implemented
    /// - Unlock mechanism is implemented
    /// - Free Will choice mechanism is implemented
    /// - Spiral development is supported
    /// - Center-archetype mapping is correct
    #[test]
    fn test_phase5_comprehensive_validation() {
        // Create all 7 energy centers
        let centers: Vec<RayCenter> = RayCenter::all();

        for ray_center in centers {
            let center = EnergyCenter::new(ray_center);

            // Criterion 1: Energy Centers are interfaces, not containers
            assert_eq!(center.ray_center, ray_center);
            assert_eq!(center.sub_colors.len(), 7);
            assert!(center.coupling.overall_balance() >= 0.0);

            // Criterion 2: Sub-colors (7 per center) are implemented
            assert_eq!(center.sub_colors.len(), 7);
            for sub_color in &center.sub_colors {
                assert!(sub_color.archetype_index < 22);
            }

            // Criterion 3: Unlock mechanism is implemented
            let mut test_center = center.clone();
            let unlock_amount = test_center.unlock(0.7);
            assert!(unlock_amount > 0.0);

            // Criterion 4: Free Will choice mechanism is implemented
            // (unlock() uses archetype_activation parameter as Free Will capacity)
            let mut test_center2 = center.clone();
            test_center2.unlock(0.0); // Zero Free Will = no unlock
            assert_eq!(test_center2.activation, 0.0);

            // Criterion 5: Spiral development is supported
            let mut test_center3 = center.clone();
            // Can unlock without perfect balance (spiral development)
            test_center3.unlock(0.9);
            assert!(test_center3.activation > 0.0);

            // Criterion 6: Center-archetype mapping is correct
            assert!(!center.archetype_mapping.is_empty());
        }
    }

    /// Test Phase 5 success metrics
    ///
    /// PHASE 5 Success Metrics:
    /// - Interface pattern: 100% implemented
    /// - Sub-colors: 100% implemented
    /// - Spiral development: 100% supported
    #[test]
    fn test_phase5_success_metrics() {
        // Metric 1: Interface pattern: 100% implemented
        let center = EnergyCenter::new(RayCenter::Green);
        assert!(center.activation >= 0.0 && center.activation <= 1.0);
        assert!(center.balance >= 0.0 && center.balance <= 1.0);
        assert_eq!(center.sub_colors.len(), 7);
        assert!(center.coupling.overall_balance() >= 0.0);

        // Metric 2: Sub-colors: 100% implemented
        for sub_color in &center.sub_colors {
            assert!(sub_color.archetype_index < 22);
            assert!(sub_color.activation >= 0.0 && sub_color.activation <= 1.0);
            assert!(sub_color.balance >= 0.0 && sub_color.balance <= 1.0);
        }

        // Metric 3: Spiral development: 100% supported
        let mut center = EnergyCenter::new(RayCenter::Violet);
        // Can unlock without perfect balance
        center.unlock(0.8);
        assert!(center.activation > 0.0);

        // Can balance lower centers first
        let mut center2 = EnergyCenter::new(RayCenter::Red);
        center2.balance_lower_centers(0.9);
        assert!(center2.balance > 0.5);
    }
}
