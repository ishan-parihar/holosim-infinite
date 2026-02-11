// Discovery Database - Phase 6: Novel Discovery, Task 6.3
//
// This module implements a database to store discovered architectures and physics.
//
// Key Components:
// - DiscoveryDatabase: Main database for storing discoveries
// - DiscoveryRecord: Individual discovery record
// - StableConfiguration: Stable holographic configuration
// - PhysicsLaws: Physical laws discovered from configurations
// - PhysicsConstant: Physical constant values
//
// Knowledge Base References:
// - REFACTOR_ROADMAP_HOLOGRAPHIC_ARCHITECTURE.md Phase 6, Task 6.3
// - "Create database to store discovered architectures and physics"

use crate::types::Float;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// ============================================================================
// DISCOVERY DATABASE
// ============================================================================

/// Discovery Database
///
/// Stores discovered architectures and physics from exploration.
#[derive(Debug, Clone)]
pub struct DiscoveryDatabase {
    /// Records of discoveries
    records: Vec<DiscoveryRecord>,

    /// Stable configurations discovered
    stable_configurations: HashMap<String, StableConfiguration>,

    /// Physical laws discovered
    physical_laws: HashMap<String, PhysicsLaws>,

    /// Physical constants discovered
    physics_constants: HashMap<String, PhysicsConstant>,

    /// Database statistics
    statistics: DatabaseStatistics,
}

/// Database statistics
#[derive(Debug, Clone, Default)]
pub struct DatabaseStatistics {
    /// Total number of discoveries
    pub total_discoveries: usize,

    /// Number of novel discoveries
    pub novel_discoveries: usize,

    /// Number of stable configurations
    pub stable_configurations: usize,

    /// Number of physical laws discovered
    pub physical_laws: usize,

    /// Number of physical constants discovered
    pub physics_constants: usize,
}

impl DiscoveryDatabase {
    /// Create a new discovery database
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            stable_configurations: HashMap::new(),
            physical_laws: HashMap::new(),
            physics_constants: HashMap::new(),
            statistics: DatabaseStatistics::default(),
        }
    }

    /// Add a discovery record
    pub fn add_discovery(&mut self, record: DiscoveryRecord) -> Result<(), String> {
        self.records.push(record.clone());
        self.statistics.total_discoveries += 1;

        if record.novelty_score > 0.7 {
            self.statistics.novel_discoveries += 1;
        }

        Ok(())
    }

    /// Add a stable configuration
    pub fn add_stable_configuration(&mut self, config: StableConfiguration) -> Result<(), String> {
        let id = config.id.clone();
        self.stable_configurations.insert(id, config);
        self.statistics.stable_configurations += 1;
        Ok(())
    }

    /// Add physical laws
    pub fn add_physical_laws(&mut self, laws: PhysicsLaws) -> Result<(), String> {
        let id = laws.id.clone();
        self.physical_laws.insert(id, laws);
        self.statistics.physical_laws += 1;
        Ok(())
    }

    /// Add a physical constant
    pub fn add_physics_constant(&mut self, constant: PhysicsConstant) -> Result<(), String> {
        let id = constant.id.clone();
        self.physics_constants.insert(id, constant);
        self.statistics.physics_constants += 1;
        Ok(())
    }

    /// Get all discovery records
    pub fn get_records(&self) -> &[DiscoveryRecord] {
        &self.records
    }

    /// Get all stable configurations
    pub fn get_stable_configurations(&self) -> &HashMap<String, StableConfiguration> {
        &self.stable_configurations
    }

    /// Get all physical laws
    pub fn get_physical_laws(&self) -> &HashMap<String, PhysicsLaws> {
        &self.physical_laws
    }

    /// Get all physics constants
    pub fn get_physics_constants(&self) -> &HashMap<String, PhysicsConstant> {
        &self.physics_constants
    }

    /// Query the database
    pub fn query(&self, query: &DiscoveryQuery) -> Vec<DiscoveryRecord> {
        self.records
            .iter()
            .filter(|record| {
                // Filter by minimum novelty score
                if let Some(min_novelty) = query.min_novelty_score {
                    if record.novelty_score < min_novelty {
                        return false;
                    }
                }

                // Filter by variant type
                if let Some(ref variant_type) = query.variant_type {
                    if record.variant_type != *variant_type {
                        return false;
                    }
                }

                // Filter by minimum stability
                if let Some(min_stability) = query.min_stability {
                    if record.stability_score < min_stability {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect()
    }

    /// Get database statistics
    pub fn get_statistics(&self) -> &DatabaseStatistics {
        &self.statistics
    }

    /// Clear the database
    pub fn clear(&mut self) {
        self.records.clear();
        self.stable_configurations.clear();
        self.physical_laws.clear();
        self.physics_constants.clear();
        self.statistics = DatabaseStatistics::default();
    }
}

// ============================================================================
// DISCOVERY RECORD
// ============================================================================

/// Discovery Record
///
/// Represents a single discovery from exploring a variant architecture.
#[derive(Debug, Clone)]
pub struct DiscoveryRecord {
    /// Unique identifier for the discovery
    pub id: String,

    /// Variant type explored
    pub variant_type: String,

    /// Description of the variant
    pub description: String,

    /// Discovered configurations
    pub discovered_configurations: Vec<String>,

    /// Novelty score (0.0 to 1.0)
    pub novelty_score: Float,

    /// Stability score (0.0 to 1.0)
    pub stability_score: Float,

    /// Physics discovered
    pub physics_discovered: Vec<String>,

    /// Timestamp of discovery
    pub timestamp: u64,
}

impl DiscoveryRecord {
    /// Create a new discovery record
    pub fn new(
        id: String,
        variant_type: String,
        description: String,
        novelty_score: Float,
        stability_score: Float,
    ) -> Self {
        Self {
            id,
            variant_type,
            description,
            discovered_configurations: Vec::new(),
            novelty_score,
            stability_score,
            physics_discovered: Vec::new(),
            timestamp: 0, // TODO: Use actual timestamp
        }
    }
}

// ============================================================================
// STABLE CONFIGURATION
// ============================================================================

/// Stable Configuration
///
/// A stable holographic configuration discovered from exploration.
#[derive(Debug, Clone)]
pub struct StableConfiguration {
    /// Unique identifier
    pub id: String,

    /// Archetype activation pattern (22 archetypes)
    pub archetype_activation: [Float; 22],

    /// Spatial frequency value
    pub spatial_frequency: Float,

    /// Resonance score (0.0 to 1.0)
    pub resonance_score: Float,

    /// Stability score (0.0 to 1.0)
    pub stability_score: Float,

    /// Physical properties derived from this configuration
    pub physical_properties: HashMap<String, Float>,

    /// Description of the configuration
    pub description: String,
}

impl StableConfiguration {
    /// Create a new stable configuration
    pub fn new(
        id: String,
        archetype_activation: [Float; 22],
        spatial_frequency: Float,
        resonance_score: Float,
        stability_score: Float,
    ) -> Self {
        Self {
            id,
            archetype_activation,
            spatial_frequency,
            resonance_score,
            stability_score,
            physical_properties: HashMap::new(),
            description: String::new(),
        }
    }

    /// Add a physical property
    pub fn add_property(&mut self, name: String, value: Float) {
        self.physical_properties.insert(name, value);
    }
}

// ============================================================================
// PHYSICS LAWS
// ============================================================================

/// Physical Laws
///
/// Physical laws discovered from holographic configurations.
#[derive(Debug, Clone)]
pub struct PhysicsLaws {
    /// Unique identifier
    pub id: String,

    /// Name of the law
    pub name: String,

    /// Mathematical formula
    pub formula: String,

    /// Description
    pub description: String,

    /// Constants used in the law
    pub constants: Vec<String>,

    /// Validity range
    pub validity_range: (Float, Float),

    /// Confidence score (0.0 to 1.0)
    pub confidence: Float,
}

impl PhysicsLaws {
    /// Create new physical laws
    pub fn new(id: String, name: String, formula: String) -> Self {
        Self {
            id,
            name,
            formula,
            description: String::new(),
            constants: Vec::new(),
            validity_range: (0.0, 1.0),
            confidence: 0.0,
        }
    }
}

// ============================================================================
// PHYSICS CONSTANT
// ============================================================================

/// Physics Constant
///
/// A physical constant discovered from holographic configurations.
#[derive(Debug, Clone)]
pub struct PhysicsConstant {
    /// Unique identifier
    pub id: String,

    /// Name of the constant
    pub name: String,

    /// Symbol
    pub symbol: String,

    /// Value
    pub value: Float,

    /// Units
    pub units: String,

    /// Uncertainty
    pub uncertainty: Float,

    /// Confidence score (0.0 to 1.0)
    pub confidence: Float,
}

impl PhysicsConstant {
    /// Create a new physics constant
    pub fn new(id: String, name: String, symbol: String, value: Float, units: String) -> Self {
        Self {
            id,
            name,
            symbol,
            value,
            units,
            uncertainty: 0.0,
            confidence: 0.0,
        }
    }
}

// ============================================================================
// NOVELTY SCORE
// ============================================================================

/// Novelty Score
///
/// Measures how novel a discovery is compared to known physics.
#[derive(Debug, Clone, Copy)]
pub struct NoveltyScore {
    /// Overall novelty (0.0 to 1.0)
    pub overall: Float,

    /// Novelty in mass values (0.0 to 1.0)
    pub mass_novelty: Float,

    /// Novelty in charge values (0.0 to 1.0)
    pub charge_novelty: Float,

    /// Novelty in spin values (0.0 to 1.0)
    pub spin_novelty: Float,

    /// Novelty in lifetime values (0.0 to 1.0)
    pub lifetime_novelty: Float,

    /// Novelty in force laws (0.0 to 1.0)
    pub force_novelty: Float,
}

impl NoveltyScore {
    /// Create a new novelty score
    pub fn new(
        overall: Float,
        mass_novelty: Float,
        charge_novelty: Float,
        spin_novelty: Float,
        lifetime_novelty: Float,
        force_novelty: Float,
    ) -> Self {
        Self {
            overall,
            mass_novelty,
            charge_novelty,
            spin_novelty,
            lifetime_novelty,
            force_novelty,
        }
    }

    /// Get average novelty
    pub fn average(&self) -> Float {
        (self.mass_novelty
            + self.charge_novelty
            + self.spin_novelty
            + self.lifetime_novelty
            + self.force_novelty)
            / 5.0
    }
}

// ============================================================================
// DISCOVERY QUERY
// ============================================================================

/// Discovery Query
///
/// Query parameters for searching the discovery database.
#[derive(Debug, Clone, Default)]
pub struct DiscoveryQuery {
    /// Minimum novelty score
    pub min_novelty_score: Option<Float>,

    /// Variant type filter
    pub variant_type: Option<String>,

    /// Minimum stability score
    pub min_stability: Option<Float>,

    /// Maximum number of results
    pub max_results: Option<usize>,
}

impl DiscoveryQuery {
    /// Create a new discovery query
    pub fn new() -> Self {
        Self::default()
    }

    /// Set minimum novelty score
    pub fn with_min_novelty(mut self, score: Float) -> Self {
        self.min_novelty_score = Some(score);
        self
    }

    /// Set variant type
    pub fn with_variant_type(mut self, variant_type: String) -> Self {
        self.variant_type = Some(variant_type);
        self
    }

    /// Set minimum stability
    pub fn with_min_stability(mut self, stability: Float) -> Self {
        self.min_stability = Some(stability);
        self
    }

    /// Set max results
    pub fn with_max_results(mut self, max: usize) -> Self {
        self.max_results = Some(max);
        self
    }
}

// ============================================================================
// THREAD-SAFE WRAPPER
// ============================================================================

/// Thread-safe discovery database
pub type SharedDiscoveryDatabase = Arc<RwLock<DiscoveryDatabase>>;

impl Default for DiscoveryDatabase {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discovery_database_creation() {
        let db = DiscoveryDatabase::new();
        assert_eq!(db.get_statistics().total_discoveries, 0);
        assert_eq!(db.get_statistics().stable_configurations, 0);
    }

    #[test]
    fn test_add_discovery() {
        let mut db = DiscoveryDatabase::new();
        let record = DiscoveryRecord::new(
            "test_discovery".to_string(),
            "test_variant".to_string(),
            "Test discovery".to_string(),
            0.8,
            0.9,
        );

        db.add_discovery(record).unwrap();
        assert_eq!(db.get_statistics().total_discoveries, 1);
        assert_eq!(db.get_statistics().novel_discoveries, 1);
    }

    #[test]
    fn test_add_stable_configuration() {
        let mut db = DiscoveryDatabase::new();
        let config =
            StableConfiguration::new("test_config".to_string(), [0.5; 22], 100.0, 0.9, 0.95);

        db.add_stable_configuration(config).unwrap();
        assert_eq!(db.get_statistics().stable_configurations, 1);
    }

    #[test]
    fn test_add_physical_laws() {
        let mut db = DiscoveryDatabase::new();
        let laws = PhysicsLaws::new(
            "test_laws".to_string(),
            "Test Laws".to_string(),
            "F = ma".to_string(),
        );

        db.add_physical_laws(laws).unwrap();
        assert_eq!(db.get_statistics().physical_laws, 1);
    }

    #[test]
    fn test_add_physics_constant() {
        let mut db = DiscoveryDatabase::new();
        let constant = PhysicsConstant::new(
            "test_constant".to_string(),
            "Test Constant".to_string(),
            "T".to_string(),
            1.0,
            "test_units".to_string(),
        );

        db.add_physics_constant(constant).unwrap();
        assert_eq!(db.get_statistics().physics_constants, 1);
    }

    #[test]
    fn test_query_database() {
        let mut db = DiscoveryDatabase::new();

        // Add some discoveries
        let record1 = DiscoveryRecord::new(
            "discovery1".to_string(),
            "variant1".to_string(),
            "Discovery 1".to_string(),
            0.9,
            0.95,
        );
        let record2 = DiscoveryRecord::new(
            "discovery2".to_string(),
            "variant2".to_string(),
            "Discovery 2".to_string(),
            0.5,
            0.6,
        );

        db.add_discovery(record1).unwrap();
        db.add_discovery(record2).unwrap();

        // Query for high novelty discoveries
        let query = DiscoveryQuery::new().with_min_novelty(0.7);
        let results = db.query(&query);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "discovery1");
    }

    #[test]
    fn test_clear_database() {
        let mut db = DiscoveryDatabase::new();
        let record = DiscoveryRecord::new(
            "test_discovery".to_string(),
            "test_variant".to_string(),
            "Test discovery".to_string(),
            0.8,
            0.9,
        );

        db.add_discovery(record).unwrap();
        assert_eq!(db.get_statistics().total_discoveries, 1);

        db.clear();
        assert_eq!(db.get_statistics().total_discoveries, 0);
    }

    #[test]
    fn test_novelty_score() {
        let novelty = NoveltyScore::new(0.8, 0.7, 0.9, 0.6, 0.8, 0.85);
        assert_eq!(novelty.overall, 0.8);
        assert!((novelty.average() - 0.77).abs() < 0.01);
    }

    #[test]
    fn test_stable_configuration() {
        let mut config =
            StableConfiguration::new("test_config".to_string(), [0.5; 22], 100.0, 0.9, 0.95);

        config.add_property("mass".to_string(), 1.0e-30);
        config.add_property("charge".to_string(), 1.0);

        assert_eq!(config.physical_properties.len(), 2);
        assert_eq!(config.physical_properties.get("mass"), Some(&1.0e-30));
    }

    #[test]
    fn test_physics_constant() {
        let constant = PhysicsConstant::new(
            "test_constant".to_string(),
            "Test Constant".to_string(),
            "T".to_string(),
            1.0,
            "test_units".to_string(),
        );

        assert_eq!(constant.id, "test_constant");
        assert_eq!(constant.value, 1.0);
        assert_eq!(constant.confidence, 0.0);
    }
}
