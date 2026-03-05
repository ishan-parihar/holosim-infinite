//! Cosmic Hierarchy - Structure of the Logos hierarchy
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "Entity properties derive from position in hierarchy
//!  Higher levels constrain lower levels correctly"

use super::logos_config::{
    GalacticLogosConfig, PlanetaryLogosConfig, PrimaryLogosConfig, SolarLogosConfig,
};
use super::HierarchyLevel;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CosmicHierarchy {
    pub primary_logos: PrimaryLogosConfig,
    pub galactic_logoi: Vec<GalacticLogosConfig>,
    pub solar_logoi: Vec<SolarLogosConfig>,
    pub planetary_logoi: Vec<PlanetaryLogosConfig>,
    pub relationships: HierarchyRelationships,
}

impl CosmicHierarchy {
    pub fn new() -> Self {
        Self {
            primary_logos: PrimaryLogosConfig::new(),
            galactic_logoi: Vec::new(),
            solar_logoi: Vec::new(),
            planetary_logoi: Vec::new(),
            relationships: HierarchyRelationships::new(),
        }
    }

    pub fn add_galactic_logos(&mut self, config: GalacticLogosConfig) -> usize {
        let id = self.galactic_logoi.len();
        self.galactic_logoi.push(config);
        self.relationships.add_galactic(id);
        id
    }

    pub fn add_solar_logos(&mut self, config: SolarLogosConfig, parent_galactic: usize) -> usize {
        let id = self.solar_logoi.len();
        self.solar_logoi.push(config);
        self.relationships.add_solar(id, parent_galactic);
        id
    }

    pub fn add_planetary_logos(
        &mut self,
        config: PlanetaryLogosConfig,
        parent_solar: usize,
    ) -> usize {
        let id = self.planetary_logoi.len();
        self.planetary_logoi.push(config);
        self.relationships.add_planetary(id, parent_solar);
        id
    }

    pub fn get_config_for_level(&self, level: HierarchyLevel) -> Option<LevelConfig> {
        match level {
            HierarchyLevel::Primary => Some(LevelConfig::Primary(self.primary_logos.clone())),
            HierarchyLevel::Galactic if !self.galactic_logoi.is_empty() => {
                Some(LevelConfig::Galactic(self.galactic_logoi[0].clone()))
            }
            HierarchyLevel::Solar if !self.solar_logoi.is_empty() => {
                Some(LevelConfig::Solar(self.solar_logoi[0].clone()))
            }
            HierarchyLevel::Planetary if !self.planetary_logoi.is_empty() => {
                Some(LevelConfig::Planetary(self.planetary_logoi[0].clone()))
            }
            _ => None,
        }
    }

    pub fn depth(&self) -> usize {
        let mut depth = 1;
        if !self.galactic_logoi.is_empty() {
            depth += 1;
        }
        if !self.solar_logoi.is_empty() {
            depth += 1;
        }
        if !self.planetary_logoi.is_empty() {
            depth += 1;
        }
        depth
    }

    pub fn total_entities_capacity(&self) -> usize {
        self.planetary_logoi
            .iter()
            .map(|p| p.inherited_capacity)
            .sum()
    }

    pub fn find_parent_chain(&self, planetary_id: usize) -> Option<(usize, usize)> {
        self.relationships.get_planetary_parent(planetary_id)
    }
}

impl Default for CosmicHierarchy {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum LevelConfig {
    Primary(PrimaryLogosConfig),
    Galactic(GalacticLogosConfig),
    Solar(SolarLogosConfig),
    Planetary(PlanetaryLogosConfig),
}

impl LevelConfig {
    pub fn level(&self) -> HierarchyLevel {
        match self {
            LevelConfig::Primary(_) => HierarchyLevel::Primary,
            LevelConfig::Galactic(_) => HierarchyLevel::Galactic,
            LevelConfig::Solar(_) => HierarchyLevel::Solar,
            LevelConfig::Planetary(_) => HierarchyLevel::Planetary,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HierarchyRelationships {
    pub solar_to_galactic: HashMap<usize, usize>,
    pub planetary_to_solar: HashMap<usize, usize>,
}

impl HierarchyRelationships {
    pub fn new() -> Self {
        Self {
            solar_to_galactic: HashMap::new(),
            planetary_to_solar: HashMap::new(),
        }
    }

    pub fn add_galactic(&mut self, _id: usize) {}

    pub fn add_solar(&mut self, solar_id: usize, parent_galactic: usize) {
        self.solar_to_galactic.insert(solar_id, parent_galactic);
    }

    pub fn add_planetary(&mut self, planetary_id: usize, parent_solar: usize) {
        self.planetary_to_solar.insert(planetary_id, parent_solar);
    }

    pub fn get_solar_parent(&self, solar_id: usize) -> Option<usize> {
        self.solar_to_galactic.get(&solar_id).copied()
    }

    pub fn get_planetary_parent(&self, planetary_id: usize) -> Option<(usize, usize)> {
        let solar_id = self.planetary_to_solar.get(&planetary_id).copied()?;
        let galactic_id = self.solar_to_galactic.get(&solar_id).copied()?;
        Some((solar_id, galactic_id))
    }

    pub fn get_children_of_galactic(&self, galactic_id: usize) -> Vec<usize> {
        self.solar_to_galactic
            .iter()
            .filter(|(_, &g)| g == galactic_id)
            .map(|(&s, _)| s)
            .collect()
    }

    pub fn get_children_of_solar(&self, solar_id: usize) -> Vec<usize> {
        self.planetary_to_solar
            .iter()
            .filter(|(_, &s)| s == solar_id)
            .map(|(&p, _)| p)
            .collect()
    }
}

impl Default for HierarchyRelationships {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct HierarchyPath {
    pub levels: Vec<HierarchyLevel>,
    pub configs: Vec<LevelConfig>,
}

impl HierarchyPath {
    pub fn new() -> Self {
        Self {
            levels: Vec::new(),
            configs: Vec::new(),
        }
    }

    pub fn push(&mut self, level: HierarchyLevel, config: LevelConfig) {
        self.levels.push(level);
        self.configs.push(config);
    }

    pub fn depth(&self) -> usize {
        self.levels.len()
    }

    pub fn inherits_from(&self, level: HierarchyLevel) -> bool {
        self.levels.contains(&level)
    }
}

impl Default for HierarchyPath {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hierarchy_creation() {
        let hierarchy = CosmicHierarchy::new();
        assert_eq!(hierarchy.depth(), 1);
    }

    #[test]
    fn test_add_logoi() {
        let mut hierarchy = CosmicHierarchy::new();

        let primary = PrimaryLogosConfig::new();
        let galactic = primary.derive_galactic_parameters();
        let solar = galactic.derive_solar_parameters();
        let planetary = solar.derive_planetary_parameters();

        hierarchy.add_galactic_logos(galactic);
        hierarchy.add_solar_logos(solar, 0);
        hierarchy.add_planetary_logos(planetary, 0);

        assert_eq!(hierarchy.depth(), 4);
        assert_eq!(hierarchy.galactic_logoi.len(), 1);
        assert_eq!(hierarchy.solar_logoi.len(), 1);
        assert_eq!(hierarchy.planetary_logoi.len(), 1);
    }

    #[test]
    fn test_parent_chain() {
        let mut hierarchy = CosmicHierarchy::new();

        let primary = PrimaryLogosConfig::new();
        let galactic = primary.derive_galactic_parameters();
        let solar = galactic.derive_solar_parameters();
        let planetary = solar.derive_planetary_parameters();

        hierarchy.add_galactic_logos(galactic);
        hierarchy.add_solar_logos(solar, 0);
        hierarchy.add_planetary_logos(planetary, 0);

        let chain = hierarchy.find_parent_chain(0);
        assert_eq!(chain, Some((0, 0)));
    }

    #[test]
    fn test_hierarchy_relationships() {
        let mut rels = HierarchyRelationships::new();

        rels.add_galactic(0);
        rels.add_solar(0, 0);
        rels.add_solar(1, 0);
        rels.add_planetary(0, 0);
        rels.add_planetary(1, 1);

        assert_eq!(rels.get_solar_parent(0), Some(0));
        assert_eq!(rels.get_planetary_parent(0), Some((0, 0)));

        let children = rels.get_children_of_galactic(0);
        assert_eq!(children.len(), 2);
    }

    #[test]
    fn test_level_config() {
        let primary = PrimaryLogosConfig::new();
        let config = LevelConfig::Primary(primary);

        assert_eq!(config.level(), HierarchyLevel::Primary);
    }
}
