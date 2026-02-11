//! Demo Scenarios
//!
//! Pre-configured simulation scenarios for demonstrations and tutorials.
//! Each scenario sets up specific initial conditions for showcasing features.
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md: "Demo scenarios"

use std::collections::HashMap;

use crate::entity_layer7::{DensityLevel, EntityId, EntityType};
use crate::foundation::{IntelligentInfinity, LightLoveField, Logos, VioletRealm};
use crate::hpo::SimulationConfig;
use crate::integrated_system::IntegratedSystem;
use crate::types::Density;

/// Demo scenario definition
///
/// A scenario configures the simulation with specific:
/// - Initial conditions
/// - Entity configurations
/// - Display settings
/// - Camera positions
/// - Tutorial integration
#[derive(Debug, Clone)]
pub struct DemoScenario {
    /// Unique identifier
    pub id: String,

    /// Display name
    pub name: String,

    /// Description
    pub description: String,

    /// Category
    pub category: ScenarioCategory,

    /// Difficulty level
    pub difficulty: DifficultyLevel,

    /// Estimated duration in minutes
    pub duration_minutes: u32,

    /// Featured entities (for highlighting)
    pub featured_entities: Vec<String>,

    /// Initial simulation configuration
    pub config: ScenarioConfig,

    /// Camera bookmarks
    pub bookmarks: Vec<ScenarioBookmark>,

    /// Tutorial to auto-start
    pub tutorial_id: Option<String>,

    /// Tips to show
    pub tips: Vec<String>,
}

/// Scenario categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScenarioCategory {
    Introduction,
    Evolution,
    Collective,
    Emergence,
    Complexity,
    Exploration,
}

/// Difficulty levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DifficultyLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

/// Scenario configuration
#[derive(Debug, Clone)]
pub struct ScenarioConfig {
    /// Number of entities
    pub entity_count: usize,

    /// Initial density distribution
    pub density_distribution: HashMap<u8, f32>, // density -> percentage

    /// Enable collective dynamics
    pub enable_collectives: bool,

    /// Enable emergence tracking
    pub enable_emergence: bool,

    /// Time scale
    pub time_scale: f32,

    /// Focus dilation enabled
    pub focus_dilation: bool,

    /// Show spectrum overlay
    pub show_spectrum: bool,

    /// Show emergence dashboard
    pub show_emergence: bool,

    /// Show collective visualization
    pub show_collectives: bool,
}

/// Scenario bookmark
#[derive(Debug, Clone)]
pub struct ScenarioBookmark {
    pub name: String,
    pub description: String,
    pub position: [f32; 3],
    pub zoom: f32,
}

/// Demo scenario manager
pub struct DemoScenarioManager {
    /// Available scenarios
    scenarios: HashMap<String, DemoScenario>,

    /// Currently loaded scenario
    current_scenario: Option<String>,

    /// Scenario groups by category
    categories: HashMap<ScenarioCategory, Vec<String>>,
}

impl DemoScenarioManager {
    /// Create new scenario manager with built-in scenarios
    pub fn new() -> Self {
        let mut manager = Self {
            scenarios: HashMap::new(),
            current_scenario: None,
            categories: HashMap::new(),
        };

        // Register built-in scenarios
        manager.register_builtin_scenarios();

        manager
    }

    /// Register all built-in scenarios
    fn register_builtin_scenarios(&mut self) {
        // Introduction scenarios
        self.register_scenario(DemoScenario {
            id: "intro_basic".to_string(),
            name: "Getting Started".to_string(),
            description:
                "Learn the basics of navigating the simulation and understanding entities."
                    .to_string(),
            category: ScenarioCategory::Introduction,
            difficulty: DifficultyLevel::Beginner,
            duration_minutes: 5,
            featured_entities: vec!["entity_001".to_string()],
            config: ScenarioConfig {
                entity_count: 16,
                density_distribution: [(1, 0.25), (2, 0.25), (3, 0.25), (4, 0.25)]
                    .iter()
                    .cloned()
                    .collect(),
                enable_collectives: false,
                enable_emergence: true,
                time_scale: 0.5,
                focus_dilation: false,
                show_spectrum: true,
                show_emergence: true,
                show_collectives: false,
            },
            bookmarks: vec![ScenarioBookmark {
                name: "Overview".to_string(),
                description: "Full view of all entities".to_string(),
                position: [0.0, 0.0, 10.0],
                zoom: 1.0,
            }],
            tutorial_id: Some("getting_started".to_string()),
            tips: vec![
                "Click on an entity to inspect it".to_string(),
                "Use the scroll wheel to zoom".to_string(),
                "Drag to pan the camera".to_string(),
            ],
        });

        self.register_scenario(DemoScenario {
            id: "intro_navigation".to_string(),
            name: "Navigation Master".to_string(),
            description: "Master the camera controls and multi-scale navigation.".to_string(),
            category: ScenarioCategory::Introduction,
            difficulty: DifficultyLevel::Beginner,
            duration_minutes: 8,
            featured_entities: vec![],
            config: ScenarioConfig {
                entity_count: 32,
                density_distribution: [(1, 0.2), (2, 0.2), (3, 0.2), (4, 0.2), (5, 0.2)]
                    .iter()
                    .cloned()
                    .collect(),
                enable_collectives: false,
                enable_emergence: false,
                time_scale: 0.1,
                focus_dilation: false,
                show_spectrum: true,
                show_emergence: false,
                show_collectives: false,
            },
            bookmarks: vec![
                ScenarioBookmark {
                    name: "Quantum Scale".to_string(),
                    description: "View at quantum level".to_string(),
                    position: [-30.0, 0.0, 0.0],
                    zoom: 0.1,
                },
                ScenarioBookmark {
                    name: "Universal Scale".to_string(),
                    description: "View at universal level".to_string(),
                    position: [25.0, 0.0, 0.0],
                    zoom: 10.0,
                },
            ],
            tutorial_id: Some("navigation".to_string()),
            tips: vec![
                "Use keys 1-5 to jump to bookmarks".to_string(),
                "Notice the logarithmic scale".to_string(),
            ],
        });

        // Evolution scenarios
        self.register_scenario(DemoScenario {
            id: "evolution_density".to_string(),
            name: "Density Octave Journey".to_string(),
            description: "Watch entities evolve through all 8 densities of the octave.".to_string(),
            category: ScenarioCategory::Evolution,
            difficulty: DifficultyLevel::Intermediate,
            duration_minutes: 15,
            featured_entities: vec!["evolver_001".to_string()],
            config: ScenarioConfig {
                entity_count: 64,
                density_distribution: [(1, 0.4), (2, 0.3), (3, 0.2), (4, 0.1)]
                    .iter()
                    .cloned()
                    .collect(),
                enable_collectives: false,
                enable_emergence: true,
                time_scale: 2.0,
                focus_dilation: true,
                show_spectrum: true,
                show_emergence: true,
                show_collectives: false,
            },
            bookmarks: vec![
                ScenarioBookmark {
                    name: "1st Density".to_string(),
                    description: "Elemental consciousness".to_string(),
                    position: [0.0, 0.0, 5.0],
                    zoom: 1.0,
                },
                ScenarioBookmark {
                    name: "4th Density".to_string(),
                    description: "Love and understanding".to_string(),
                    position: [0.0, 0.0, 5.0],
                    zoom: 1.0,
                },
            ],
            tutorial_id: None,
            tips: vec![
                "Watch entities change color as they evolve".to_string(),
                "Notice the emergence events".to_string(),
            ],
        });

        self.register_scenario(DemoScenario {
            id: "evolution_spectrum".to_string(),
            name: "Space/Time vs Time/Space".to_string(),
            description: "Explore the spectrum between space/time and time/space.".to_string(),
            category: ScenarioCategory::Evolution,
            difficulty: DifficultyLevel::Intermediate,
            duration_minutes: 12,
            featured_entities: vec![],
            config: ScenarioConfig {
                entity_count: 48,
                density_distribution: [(3, 0.3), (4, 0.4), (5, 0.3)].iter().cloned().collect(),
                enable_collectives: false,
                enable_emergence: false,
                time_scale: 0.5,
                focus_dilation: false,
                show_spectrum: true,
                show_emergence: false,
                show_collectives: false,
            },
            bookmarks: vec![],
            tutorial_id: None,
            tips: vec![
                "Entities on the left are space/time oriented".to_string(),
                "Entities on the right are time/space oriented".to_string(),
                "The veil is at the center (v=1)".to_string(),
            ],
        });

        // Collective scenarios
        self.register_scenario(DemoScenario {
            id: "collective_formation".to_string(),
            name: "Birth of Collectives".to_string(),
            description: "Watch how entities form groups based on resonance.".to_string(),
            category: ScenarioCategory::Collective,
            difficulty: DifficultyLevel::Intermediate,
            duration_minutes: 10,
            featured_entities: vec![],
            config: ScenarioConfig {
                entity_count: 128,
                density_distribution: [(2, 0.25), (3, 0.25), (4, 0.25), (5, 0.25)]
                    .iter()
                    .cloned()
                    .collect(),
                enable_collectives: true,
                enable_emergence: false,
                time_scale: 1.0,
                focus_dilation: false,
                show_spectrum: false,
                show_emergence: false,
                show_collectives: true,
            },
            bookmarks: vec![],
            tutorial_id: None,
            tips: vec![
                "Watch entities cluster by resonance".to_string(),
                "Collectives are shown as transparent regions".to_string(),
            ],
        });

        self.register_scenario(DemoScenario {
            id: "collective_dynamics".to_string(),
            name: "Collective Consciousness".to_string(),
            description: "Explore group behavior and collective intelligence.".to_string(),
            category: ScenarioCategory::Collective,
            difficulty: DifficultyLevel::Advanced,
            duration_minutes: 15,
            featured_entities: vec![],
            config: ScenarioConfig {
                entity_count: 256,
                density_distribution: [(3, 0.2), (4, 0.3), (5, 0.3), (6, 0.2)]
                    .iter()
                    .cloned()
                    .collect(),
                enable_collectives: true,
                enable_emergence: true,
                time_scale: 1.5,
                focus_dilation: true,
                show_spectrum: true,
                show_emergence: true,
                show_collectives: true,
            },
            bookmarks: vec![],
            tutorial_id: None,
            tips: vec![
                "Larger collectives show higher coherence".to_string(),
                "Watch the collective dashboard for metrics".to_string(),
            ],
        });

        // Emergence scenarios
        self.register_scenario(DemoScenario {
            id: "emergence_biological".to_string(),
            name: "Biological Emergence".to_string(),
            description: "Watch life emerge from simple chemical interactions.".to_string(),
            category: ScenarioCategory::Emergence,
            difficulty: DifficultyLevel::Intermediate,
            duration_minutes: 12,
            featured_entities: vec![],
            config: ScenarioConfig {
                entity_count: 96,
                density_distribution: [(1, 0.3), (2, 0.5), (3, 0.2)].iter().cloned().collect(),
                enable_collectives: false,
                enable_emergence: true,
                time_scale: 3.0,
                focus_dilation: true,
                show_spectrum: false,
                show_emergence: true,
                show_collectives: false,
            },
            bookmarks: vec![],
            tutorial_id: None,
            tips: vec![
                "Watch for emergence events in the dashboard".to_string(),
                "Species count will increase as life emerges".to_string(),
            ],
        });

        self.register_scenario(DemoScenario {
            id: "emergence_gaia".to_string(),
            name: "Gaia Awakening".to_string(),
            description: "Witness the emergence of planetary consciousness.".to_string(),
            category: ScenarioCategory::Emergence,
            difficulty: DifficultyLevel::Advanced,
            duration_minutes: 20,
            featured_entities: vec![],
            config: ScenarioConfig {
                entity_count: 512,
                density_distribution: [(2, 0.1), (3, 0.2), (4, 0.3), (5, 0.3), (6, 0.1)]
                    .iter()
                    .cloned()
                    .collect(),
                enable_collectives: true,
                enable_emergence: true,
                time_scale: 2.0,
                focus_dilation: true,
                show_spectrum: true,
                show_emergence: true,
                show_collectives: true,
            },
            bookmarks: vec![],
            tutorial_id: None,
            tips: vec![
                "Watch the Gaia level in the emergence dashboard".to_string(),
                "Global consciousness emerges gradually".to_string(),
            ],
        });

        // Complexity scenarios
        self.register_scenario(DemoScenario {
            id: "complexity_structure".to_string(),
            name: "Hierarchical Structure".to_string(),
            description: "Explore the nested structure from atoms to galaxies.".to_string(),
            category: ScenarioCategory::Complexity,
            difficulty: DifficultyLevel::Advanced,
            duration_minutes: 15,
            featured_entities: vec![],
            config: ScenarioConfig {
                entity_count: 384,
                density_distribution: [
                    (1, 0.15),
                    (2, 0.15),
                    (3, 0.2),
                    (4, 0.2),
                    (5, 0.2),
                    (6, 0.1),
                ]
                .iter()
                .cloned()
                .collect(),
                enable_collectives: true,
                enable_emergence: true,
                time_scale: 1.0,
                focus_dilation: true,
                show_spectrum: true,
                show_emergence: true,
                show_collectives: true,
            },
            bookmarks: vec![
                ScenarioBookmark {
                    name: "Atomic".to_string(),
                    description: "Atomic scale structures".to_string(),
                    position: [-10.0, 0.0, 0.0],
                    zoom: 0.01,
                },
                ScenarioBookmark {
                    name: "Organism".to_string(),
                    description: "Life forms".to_string(),
                    position: [-2.0, 0.0, 0.0],
                    zoom: 0.1,
                },
                ScenarioBookmark {
                    name: "Planetary".to_string(),
                    description: "Planetary systems".to_string(),
                    position: [6.0, 0.0, 0.0],
                    zoom: 1.0,
                },
                ScenarioBookmark {
                    name: "Galactic".to_string(),
                    description: "Galaxy structures".to_string(),
                    position: [20.0, 0.0, 0.0],
                    zoom: 10.0,
                },
            ],
            tutorial_id: None,
            tips: vec![
                "Use the structure dashboard to explore hierarchies".to_string(),
                "Notice how entities form nested structures".to_string(),
            ],
        });

        // Exploration scenarios
        self.register_scenario(DemoScenario {
            id: "exploration_full".to_string(),
            name: "Cosmic Explorer".to_string(),
            description: "Freely explore a fully-featured simulation.".to_string(),
            category: ScenarioCategory::Exploration,
            difficulty: DifficultyLevel::Expert,
            duration_minutes: 30,
            featured_entities: vec![],
            config: ScenarioConfig {
                entity_count: 1024,
                density_distribution: [
                    (1, 0.125),
                    (2, 0.125),
                    (3, 0.125),
                    (4, 0.125),
                    (5, 0.125),
                    (6, 0.125),
                    (7, 0.125),
                    (8, 0.125),
                ]
                .iter()
                .cloned()
                .collect(),
                enable_collectives: true,
                enable_emergence: true,
                time_scale: 1.0,
                focus_dilation: true,
                show_spectrum: true,
                show_emergence: true,
                show_collectives: true,
            },
            bookmarks: vec![
                ScenarioBookmark {
                    name: "Quantum Realm".to_string(),
                    description: "10^-35 to 10^-15 meters".to_string(),
                    position: [-25.0, 0.0, 0.0],
                    zoom: 0.001,
                },
                ScenarioBookmark {
                    name: "Solar System".to_string(),
                    description: "10^11 to 10^13 meters".to_string(),
                    position: [12.0, 0.0, 0.0],
                    zoom: 100.0,
                },
                ScenarioBookmark {
                    name: "Galaxy".to_string(),
                    description: "10^20 to 10^22 meters".to_string(),
                    position: [21.0, 0.0, 0.0],
                    zoom: 10000.0,
                },
                ScenarioBookmark {
                    name: "Observable Universe".to_string(),
                    description: "10^26 meters".to_string(),
                    position: [26.0, 0.0, 0.0],
                    zoom: 100000.0,
                },
            ],
            tutorial_id: None,
            tips: vec![
                "Explore all features at your own pace".to_string(),
                "Try all the different visualizations".to_string(),
                "Experiment with time controls".to_string(),
            ],
        });
    }

    /// Register a scenario
    pub fn register_scenario(&mut self, scenario: DemoScenario) {
        let category = scenario.category;
        let id = scenario.id.clone();

        self.scenarios.insert(id.clone(), scenario);

        self.categories
            .entry(category)
            .or_insert_with(Vec::new)
            .push(id);
    }

    /// Get a scenario by ID
    pub fn get_scenario(&self, id: &str) -> Option<&DemoScenario> {
        self.scenarios.get(id)
    }

    /// Get all scenarios
    pub fn get_all_scenarios(&self) -> Vec<&DemoScenario> {
        self.scenarios.values().collect()
    }

    /// Get scenarios by category
    pub fn get_scenarios_by_category(&self, category: ScenarioCategory) -> Vec<&DemoScenario> {
        self.categories
            .get(&category)
            .map(|ids| ids.iter().filter_map(|id| self.scenarios.get(id)).collect())
            .unwrap_or_default()
    }

    /// Get scenarios by difficulty
    pub fn get_scenarios_by_difficulty(&self, difficulty: DifficultyLevel) -> Vec<&DemoScenario> {
        self.scenarios
            .values()
            .filter(|s| s.difficulty == difficulty)
            .collect()
    }

    /// Load a scenario
    pub fn load_scenario(&mut self, id: &str) -> Option<DemoScenario> {
        if let Some(scenario) = self.scenarios.get(id).cloned() {
            self.current_scenario = Some(id.to_string());
            Some(scenario)
        } else {
            None
        }
    }

    /// Get currently loaded scenario
    pub fn current_scenario(&self) -> Option<&DemoScenario> {
        self.current_scenario
            .as_ref()
            .and_then(|id| self.scenarios.get(id))
    }

    /// Clear current scenario
    pub fn clear_current(&mut self) {
        self.current_scenario = None;
    }

    /// Get available categories
    pub fn get_categories(&self) -> Vec<ScenarioCategory> {
        self.categories.keys().copied().collect()
    }

    /// Get category name
    pub fn get_category_name(category: ScenarioCategory) -> &'static str {
        match category {
            ScenarioCategory::Introduction => "Introduction",
            ScenarioCategory::Evolution => "Evolution",
            ScenarioCategory::Collective => "Collective",
            ScenarioCategory::Emergence => "Emergence",
            ScenarioCategory::Complexity => "Complexity",
            ScenarioCategory::Exploration => "Exploration",
        }
    }

    /// Get difficulty name
    pub fn get_difficulty_name(difficulty: DifficultyLevel) -> &'static str {
        match difficulty {
            DifficultyLevel::Beginner => "Beginner",
            DifficultyLevel::Intermediate => "Intermediate",
            DifficultyLevel::Advanced => "Advanced",
            DifficultyLevel::Expert => "Expert",
        }
    }
}

impl Default for DemoScenarioManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scenario_manager_new() {
        let manager = DemoScenarioManager::new();

        // Should have built-in scenarios
        assert!(!manager.scenarios.is_empty());
        assert!(!manager.categories.is_empty());
    }

    #[test]
    fn test_get_scenario() {
        let manager = DemoScenarioManager::new();

        let scenario = manager.get_scenario("intro_basic");
        assert!(scenario.is_some());

        let scenario = scenario.unwrap();
        assert_eq!(scenario.id, "intro_basic");
        assert_eq!(scenario.category, ScenarioCategory::Introduction);
    }

    #[test]
    fn test_get_scenarios_by_category() {
        let manager = DemoScenarioManager::new();

        let intro_scenarios = manager.get_scenarios_by_category(ScenarioCategory::Introduction);
        assert!(!intro_scenarios.is_empty());

        for scenario in &intro_scenarios {
            assert_eq!(scenario.category, ScenarioCategory::Introduction);
        }
    }

    #[test]
    fn test_get_scenarios_by_difficulty() {
        let manager = DemoScenarioManager::new();

        let beginner = manager.get_scenarios_by_difficulty(DifficultyLevel::Beginner);
        assert!(!beginner.is_empty());
    }

    #[test]
    fn test_load_scenario() {
        let mut manager = DemoScenarioManager::new();

        assert!(manager.current_scenario.is_none());

        let scenario = manager.load_scenario("intro_basic");
        assert!(scenario.is_some());
        assert!(manager.current_scenario.is_some());

        manager.clear_current();
        assert!(manager.current_scenario.is_none());
    }

    #[test]
    fn test_category_names() {
        assert_eq!(
            DemoScenarioManager::get_category_name(ScenarioCategory::Introduction),
            "Introduction"
        );
        assert_eq!(
            DemoScenarioManager::get_category_name(ScenarioCategory::Evolution),
            "Evolution"
        );
    }

    #[test]
    fn test_difficulty_names() {
        assert_eq!(
            DemoScenarioManager::get_difficulty_name(DifficultyLevel::Beginner),
            "Beginner"
        );
        assert_eq!(
            DemoScenarioManager::get_difficulty_name(DifficultyLevel::Expert),
            "Expert"
        );
    }

    #[test]
    fn test_scenario_config_default() {
        let config = ScenarioConfig {
            entity_count: 100,
            density_distribution: HashMap::new(),
            enable_collectives: true,
            enable_emergence: true,
            time_scale: 1.0,
            focus_dilation: false,
            show_spectrum: true,
            show_emergence: true,
            show_collectives: true,
        };

        assert_eq!(config.entity_count, 100);
        assert!(config.enable_collectives);
    }
}
