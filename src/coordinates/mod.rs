// Coordinates System - Phase 3: Green-Ray Realm
// Implements Space/Time and Time/Space dual coordinate systems
//
// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 2.3
// "Space/Time: The physical realm where entities experience separation,
// limitation, and linear time"
// "Time/Space: The metaphysical realm where entities review experiences,
// plan incarnations, and access broader consciousness"

pub mod navigation;
pub mod space_time;
pub mod time_space;
pub mod translation;

pub use navigation::{
    DualRealmNavigator, EntityRealm, NavigationResult, NavigationStatistics, SpaceTimeRealm,
    TimeSpaceRealm,
};
pub use space_time::{PhysicalSpaceTimeCoord, PlanetarySystemId};
pub use time_space::{
    BroaderConsciousnessAccess, ExperienceVector, IncarnationPlan, MetaphysicalTimeSpaceCoord,
};
pub use translation::{RealmTranslation, TRANSLATION_THRESHOLD};
