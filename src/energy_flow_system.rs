// Energy Flow System - Phase 1.2 Implementation
// Implements hierarchical energy flow from IntelligentInfinity through density levels
// with resource constraints and competition mechanisms

use crate::types::Float;
use crate::types::HolonID;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Energy Source in the hierarchical flow
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnergySource {
    /// IntelligentInfinity - The source of all energy (Density 8, beyond manifestation)
    IntelligentInfinity,
    /// Cosmic Logos - First distortion: Free Will (Density 7)
    CosmicLogos,
    /// Sub-Logos - Second distortion: Love (Density 6)
    SubLogos,
    /// Logos - Third distortion: Light (Density 5)
    Logos,
    /// Co-Creators - Begin involution into matter (Density 4)
    CoCreators,
    /// Physical Entities - Manifestation (Densities 1-3)
    PhysicalEntities,
}

impl EnergySource {
    /// Get the hierarchy level (higher = closer to source)
    pub fn hierarchy_level(&self) -> u8 {
        match self {
            EnergySource::IntelligentInfinity => 6,
            EnergySource::CosmicLogos => 5,
            EnergySource::SubLogos => 4,
            EnergySource::Logos => 3,
            EnergySource::CoCreators => 2,
            EnergySource::PhysicalEntities => 1,
        }
    }

    /// Get the density associated with this energy source
    pub fn density(&self) -> Option<u8> {
        match self {
            EnergySource::IntelligentInfinity => Some(8),
            EnergySource::CosmicLogos => Some(7),
            EnergySource::SubLogos => Some(6),
            EnergySource::Logos => Some(5),
            EnergySource::CoCreators => Some(4),
            EnergySource::PhysicalEntities => None, // Can be D1-D3
        }
    }

    /// Get the next level in the hierarchy (toward manifestation)
    pub fn next_level(&self) -> Option<EnergySource> {
        match self {
            EnergySource::IntelligentInfinity => Some(EnergySource::CosmicLogos),
            EnergySource::CosmicLogos => Some(EnergySource::SubLogos),
            EnergySource::SubLogos => Some(EnergySource::Logos),
            EnergySource::Logos => Some(EnergySource::CoCreators),
            EnergySource::CoCreators => Some(EnergySource::PhysicalEntities),
            EnergySource::PhysicalEntities => None,
        }
    }

    /// Get the previous level in the hierarchy (toward source)
    pub fn previous_level(&self) -> Option<EnergySource> {
        match self {
            EnergySource::IntelligentInfinity => None,
            EnergySource::CosmicLogos => Some(EnergySource::IntelligentInfinity),
            EnergySource::SubLogos => Some(EnergySource::CosmicLogos),
            EnergySource::Logos => Some(EnergySource::SubLogos),
            EnergySource::CoCreators => Some(EnergySource::Logos),
            EnergySource::PhysicalEntities => Some(EnergySource::CoCreators),
        }
    }
}

/// Energy pool at a specific hierarchy level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyPool {
    /// The energy source for this pool
    pub source: EnergySource,

    /// Total energy available in this pool (limited resource)
    pub total_energy: Float,

    /// Energy currently allocated to entities
    pub allocated_energy: Float,

    /// Energy currently available for allocation
    pub available_energy: Float,

    /// Number of entities competing for this energy
    pub competitor_count: usize,

    /// Hierarchy level factor (higher levels get more energy per entity)
    pub hierarchy_factor: Float,
}

impl EnergyPool {
    /// Create a new energy pool
    pub fn new(source: EnergySource, total_energy: Float) -> Self {
        let hierarchy_factor = Self::calculate_hierarchy_factor(source);

        EnergyPool {
            source,
            total_energy,
            allocated_energy: 0.0,
            available_energy: total_energy,
            competitor_count: 0,
            hierarchy_factor,
        }
    }

    /// Calculate hierarchy factor based on energy source
    /// Higher levels get more energy per entity
    fn calculate_hierarchy_factor(source: EnergySource) -> Float {
        match source {
            EnergySource::IntelligentInfinity => 10.0, // Infinite potential
            EnergySource::CosmicLogos => 5.0,
            EnergySource::SubLogos => 3.0,
            EnergySource::Logos => 2.0,
            EnergySource::CoCreators => 1.5,
            EnergySource::PhysicalEntities => 1.0, // Base level
        }
    }

    /// Get the percentage of energy allocated
    pub fn allocation_percentage(&self) -> Float {
        if self.total_energy > 0.0 {
            (self.allocated_energy / self.total_energy) * 100.0
        } else {
            0.0
        }
    }

    /// Check if energy is available
    pub fn has_energy(&self) -> bool {
        self.available_energy > 0.0
    }

    /// Allocate energy to an entity
    pub fn allocate(&mut self, amount: Float) -> Result<Float, String> {
        if amount <= 0.0 {
            return Err("Amount must be positive".to_string());
        }

        if amount > self.available_energy {
            // Allocate what's available
            let allocated = self.available_energy;
            self.allocated_energy += allocated;
            self.available_energy = 0.0;
            Ok(allocated)
        } else {
            self.allocated_energy += amount;
            self.available_energy -= amount;
            Ok(amount)
        }
    }

    /// Reset energy pool for new cycle
    pub fn reset(&mut self) {
        self.allocated_energy = 0.0;
        self.available_energy = self.total_energy;
        self.competitor_count = 0;
    }

    /// Replenish energy pool (from higher level)
    pub fn replenish(&mut self, amount: Float) {
        self.total_energy += amount;
        self.available_energy += amount;
    }
}

/// Entity energy request for competition
#[derive(Debug, Clone)]
pub struct EnergyRequest {
    /// Entity ID
    pub entity_id: HolonID,

    /// Entity's density (1-7)
    pub density: u8,

    /// Entity's developmental level (0.0 to 1.0)
    pub developmental_level: Float,

    /// Entity's polarization intensity (0.0 to 1.0)
    pub polarization_intensity: Float,

    /// Entity's resonance with source (0.0 to 1.0)
    pub resonance: Float,

    /// Entity's energy need (0.0 to 1.0)
    pub energy_need: Float,

    /// Entity's free will capacity (0.0 to 1.0)
    pub free_will_capacity: Float,

    /// Calculated energy share (will be filled during distribution)
    pub energy_share: Float,
}

impl EnergyRequest {
    /// Create a new energy request
    pub fn new(
        entity_id: HolonID,
        density: u8,
        developmental_level: Float,
        polarization_intensity: Float,
        resonance: Float,
        energy_need: Float,
        free_will_capacity: Float,
    ) -> Self {
        EnergyRequest {
            entity_id,
            density,
            developmental_level,
            polarization_intensity,
            resonance,
            energy_need,
            free_will_capacity,
            energy_share: 0.0,
        }
    }

    /// Calculate priority score for energy allocation
    /// Higher score = higher priority for energy
    pub fn calculate_priority(&self) -> Float {
        // Factors:
        // 1. Developmental level (more developed = higher priority)
        // 2. Polarity intensity (more polarized = higher priority)
        // 3. Resonance (higher resonance = higher priority)
        // 4. Energy need (higher need = higher priority)
        // 5. Free will capacity (higher capacity = higher priority)

        let dev_factor = self.developmental_level * 1.5;
        let pol_factor = self.polarization_intensity * 1.3;
        let res_factor = self.resonance * 1.2;
        let need_factor = self.energy_need * 1.4;
        let will_factor = self.free_will_capacity * 1.0;

        dev_factor + pol_factor + res_factor + need_factor + will_factor
    }

    /// Calculate energy entitlement based on priority
    pub fn calculate_entitlement(&self, total_energy: Float, total_priority: Float) -> Float {
        if total_priority > 0.0 {
            let priority = self.calculate_priority();
            (priority / total_priority) * total_energy
        } else {
            0.0
        }
    }
}

/// Energy flow statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyFlowStats {
    /// Total energy flowing through the system
    pub total_energy_flow: Float,

    /// Energy distribution by level
    pub energy_by_level: HashMap<EnergySource, Float>,

    /// Number of entities served
    pub entities_served: usize,

    /// Number of entities that received no energy
    pub entities_unserved: usize,

    /// Competition intensity (0.0 to 1.0)
    pub competition_intensity: Float,

    /// Average energy per entity
    pub average_energy_per_entity: Float,

    /// Energy efficiency (allocated / requested)
    pub energy_efficiency: Float,
}

impl Default for EnergyFlowStats {
    fn default() -> Self {
        EnergyFlowStats {
            total_energy_flow: 0.0,
            energy_by_level: HashMap::new(),
            entities_served: 0,
            entities_unserved: 0,
            competition_intensity: 0.0,
            average_energy_per_entity: 0.0,
            energy_efficiency: 0.0,
        }
    }
}

/// Hierarchical Energy Flow System
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyFlowSystem {
    /// Energy pools at each hierarchy level
    pub energy_pools: HashMap<EnergySource, EnergyPool>,

    /// Current statistics
    pub stats: EnergyFlowStats,

    /// Total input energy from IntelligentInfinity
    pub input_energy: Float,

    /// Energy transformation efficiency (potential to kinetic)
    pub transformation_efficiency: Float,
}

impl EnergyFlowSystem {
    /// Create a new energy flow system
    pub fn new(input_energy: Float) -> Self {
        let mut energy_pools = HashMap::new();

        // Create energy pool at each level
        // Energy flows from top to bottom, with decreasing amounts
        let cosmic_logos_energy = input_energy * 0.8;
        let sub_logos_energy = cosmic_logos_energy * 0.8;
        let logos_energy = sub_logos_energy * 0.8;
        let co_creators_energy = logos_energy * 0.8;
        let physical_energy = co_creators_energy * 0.8;

        energy_pools.insert(
            EnergySource::CosmicLogos,
            EnergyPool::new(EnergySource::CosmicLogos, cosmic_logos_energy),
        );
        energy_pools.insert(
            EnergySource::SubLogos,
            EnergyPool::new(EnergySource::SubLogos, sub_logos_energy),
        );
        energy_pools.insert(
            EnergySource::Logos,
            EnergyPool::new(EnergySource::Logos, logos_energy),
        );
        energy_pools.insert(
            EnergySource::CoCreators,
            EnergyPool::new(EnergySource::CoCreators, co_creators_energy),
        );
        energy_pools.insert(
            EnergySource::PhysicalEntities,
            EnergyPool::new(EnergySource::PhysicalEntities, physical_energy),
        );

        EnergyFlowSystem {
            energy_pools,
            stats: EnergyFlowStats::default(),
            input_energy,
            transformation_efficiency: 0.95, // 95% efficiency
        }
    }

    /// Get energy pool for a specific source
    pub fn get_pool(&self, source: EnergySource) -> Option<&EnergyPool> {
        self.energy_pools.get(&source)
    }

    /// Get mutable energy pool for a specific source
    pub fn get_pool_mut(&mut self, source: EnergySource) -> Option<&mut EnergyPool> {
        self.energy_pools.get_mut(&source)
    }

    /// Distribute energy to entities based on requests
    pub fn distribute_energy(&mut self, requests: &mut Vec<EnergyRequest>) -> EnergyFlowStats {
        let mut stats = EnergyFlowStats::default();
        stats.total_energy_flow = self.input_energy;

        // Group requests by density to determine which pool they draw from
        let mut requests_by_density: HashMap<u8, Vec<usize>> = HashMap::new();
        for (idx, req) in requests.iter().enumerate() {
            requests_by_density
                .entry(req.density)
                .or_insert_with(Vec::new)
                .push(idx);
        }

        // Distribute energy to each density group
        for (density, request_indices) in requests_by_density {
            let source = match density {
                7 => EnergySource::CosmicLogos,
                6 => EnergySource::SubLogos,
                5 => EnergySource::Logos,
                4 => EnergySource::CoCreators,
                1..=3 => EnergySource::PhysicalEntities,
                _ => EnergySource::PhysicalEntities, // Default to physical
            };

            if let Some(pool) = self.get_pool_mut(source) {
                pool.competitor_count = request_indices.len();

                // Calculate total priority for this group
                let total_priority: Float = request_indices
                    .iter()
                    .map(|&idx| requests[idx].calculate_priority())
                    .sum();

                // Distribute energy based on priority
                for &idx in &request_indices {
                    let req = &mut requests[idx];
                    let pool = self.get_pool_mut(source).unwrap();

                    // Calculate entitlement
                    let entitlement =
                        req.calculate_entitlement(pool.available_energy, total_priority);

                    // Allocate energy (may be less than entitlement due to scarcity)
                    let allocated = pool.allocate(entitlement).unwrap_or(0.0);
                    req.energy_share = allocated;

                    // Track statistics
                    if allocated > 0.0 {
                        stats.entities_served += 1;
                    } else {
                        stats.entities_unserved += 1;
                    }

                    // Track energy by level
                    *stats.energy_by_level.entry(source).or_insert(0.0) += allocated;
                }
            }
        }

        // Calculate derived statistics
        let total_entities = requests.len();
        if total_entities > 0 {
            stats.average_energy_per_entity = stats.total_energy_flow / total_entities as Float;
            stats.competition_intensity = if stats.entities_served > 0 {
                1.0 - (stats.entities_served as Float / total_entities as Float)
            } else {
                1.0
            };
        }

        // Calculate energy efficiency
        let total_requested: Float = requests.iter().map(|r| r.energy_need).sum();
        let total_allocated: Float = requests.iter().map(|r| r.energy_share).sum();
        stats.energy_efficiency = if total_requested > 0.0 {
            total_allocated / total_requested
        } else {
            0.0
        };

        self.stats = stats.clone();
        stats
    }

    /// Get energy for a specific entity (based on density and factors)
    pub fn get_energy_for_entity(
        &mut self,
        entity_id: HolonID,
        density: u8,
        developmental_level: Float,
        polarization_intensity: Float,
        resonance: Float,
        energy_need: Float,
        free_will_capacity: Float,
    ) -> Float {
        // Determine which energy pool to use
        let _source = match density {
            7 => EnergySource::CosmicLogos,
            6 => EnergySource::SubLogos,
            5 => EnergySource::Logos,
            4 => EnergySource::CoCreators,
            1..=3 => EnergySource::PhysicalEntities,
            _ => EnergySource::PhysicalEntities,
        };

        // Create energy request
        let request = EnergyRequest::new(
            entity_id,
            density,
            developmental_level,
            polarization_intensity,
            resonance,
            energy_need,
            free_will_capacity,
        );

        // Distribute energy (single entity)
        let mut requests = vec![request];
        self.distribute_energy(&mut requests);

        requests[0].energy_share
    }

    /// Reset energy pools for new cycle
    pub fn reset(&mut self) {
        for pool in self.energy_pools.values_mut() {
            pool.reset();
        }
        self.stats = EnergyFlowStats::default();
    }

    /// Get summary of energy flow
    pub fn summary(&self) -> String {
        let mut summary = String::new();
        summary.push_str("Energy Flow System Summary:\n");
        summary.push_str(&format!("  Input Energy: {:.2}\n", self.input_energy));
        summary.push_str(&format!(
            "  Transformation Efficiency: {:.2}%\n",
            self.transformation_efficiency * 100.0
        ));
        summary.push_str("\nEnergy Pools:\n");

        for source in [
            EnergySource::CosmicLogos,
            EnergySource::SubLogos,
            EnergySource::Logos,
            EnergySource::CoCreators,
            EnergySource::PhysicalEntities,
        ] {
            if let Some(pool) = self.get_pool(source) {
                summary.push_str(&format!(
                    "  {:?}: Total={:.2}, Allocated={:.2} ({:.1}%), Available={:.2}, Competitors={}\n",
                    source,
                    pool.total_energy,
                    pool.allocated_energy,
                    pool.allocation_percentage(),
                    pool.available_energy,
                    pool.competitor_count
                ));
            }
        }

        summary.push_str("\nStatistics:\n");
        summary.push_str(&format!(
            "  Entities Served: {}\n",
            self.stats.entities_served
        ));
        summary.push_str(&format!(
            "  Entities Unserved: {}\n",
            self.stats.entities_unserved
        ));
        summary.push_str(&format!(
            "  Competition Intensity: {:.2}\n",
            self.stats.competition_intensity
        ));
        summary.push_str(&format!(
            "  Average Energy per Entity: {:.2}\n",
            self.stats.average_energy_per_entity
        ));
        summary.push_str(&format!(
            "  Energy Efficiency: {:.2}%\n",
            self.stats.energy_efficiency * 100.0
        ));

        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_source_hierarchy_level() {
        assert_eq!(EnergySource::IntelligentInfinity.hierarchy_level(), 6);
        assert_eq!(EnergySource::CosmicLogos.hierarchy_level(), 5);
        assert_eq!(EnergySource::PhysicalEntities.hierarchy_level(), 1);
    }

    #[test]
    fn test_energy_source_next_level() {
        assert_eq!(
            EnergySource::IntelligentInfinity.next_level(),
            Some(EnergySource::CosmicLogos)
        );
        assert_eq!(
            EnergySource::CosmicLogos.next_level(),
            Some(EnergySource::SubLogos)
        );
        assert_eq!(EnergySource::PhysicalEntities.next_level(), None);
    }

    #[test]
    fn test_energy_pool_creation() {
        let pool = EnergyPool::new(EnergySource::CosmicLogos, 1000.0);
        assert_eq!(pool.total_energy, 1000.0);
        assert_eq!(pool.available_energy, 1000.0);
        assert_eq!(pool.allocated_energy, 0.0);
        assert_eq!(pool.hierarchy_factor, 5.0);
    }

    #[test]
    fn test_energy_pool_allocate() {
        let mut pool = EnergyPool::new(EnergySource::CosmicLogos, 1000.0);

        // Allocate 500
        let allocated = pool.allocate(500.0).unwrap();
        assert_eq!(allocated, 500.0);
        assert_eq!(pool.available_energy, 500.0);
        assert_eq!(pool.allocated_energy, 500.0);

        // Allocate more than available
        let allocated = pool.allocate(600.0).unwrap();
        assert_eq!(allocated, 500.0);
        assert_eq!(pool.available_energy, 0.0);
        assert_eq!(pool.allocated_energy, 1000.0);
    }

    #[test]
    fn test_energy_pool_reset() {
        let mut pool = EnergyPool::new(EnergySource::CosmicLogos, 1000.0);
        pool.allocate(500.0).unwrap();
        pool.competitor_count = 10;

        pool.reset();

        assert_eq!(pool.available_energy, 1000.0);
        assert_eq!(pool.allocated_energy, 0.0);
        assert_eq!(pool.competitor_count, 0);
    }

    #[test]
    fn test_energy_request_priority() {
        let request1 = EnergyRequest::new(
            1, 3, 0.8, // high developmental
            0.9, // high polarization
            0.7, // high resonance
            0.6, // moderate need
            0.5,
        );

        let request2 = EnergyRequest::new(
            2, 3, 0.2, // low developmental
            0.1, // low polarization
            0.3, // low resonance
            0.9, // high need
            0.5,
        );

        let priority1 = request1.calculate_priority();
        let priority2 = request2.calculate_priority();

        assert!(priority1 > priority2);
    }

    #[test]
    fn test_energy_flow_system_creation() {
        let system = EnergyFlowSystem::new(10000.0);

        assert_eq!(system.input_energy, 10000.0);
        assert_eq!(system.transformation_efficiency, 0.95);
        assert_eq!(system.energy_pools.len(), 5);
    }

    #[test]
    fn test_energy_flow_system_distribute() {
        let mut system = EnergyFlowSystem::new(10000.0);

        // Create requests
        let mut requests = vec![
            EnergyRequest::new(1, 3, 0.8, 0.9, 0.7, 0.5, 0.8),
            EnergyRequest::new(2, 3, 0.5, 0.6, 0.5, 0.4, 0.6),
            EnergyRequest::new(3, 3, 0.3, 0.2, 0.3, 0.3, 0.4),
        ];

        let stats = system.distribute_energy(&mut requests);

        assert_eq!(stats.entities_served, 3);
        assert!(stats.total_energy_flow > 0.0);

        // Check that higher priority entities got more energy
        assert!(requests[0].energy_share >= requests[1].energy_share);
        assert!(requests[1].energy_share >= requests[2].energy_share);
    }

    #[test]
    fn test_energy_flow_system_resource_constraints() {
        let mut system = EnergyFlowSystem::new(1000.0);

        // Create many requests to test resource constraints
        let mut requests = Vec::new();
        for i in 0..100 {
            requests.push(EnergyRequest::new(
                i, 3, 0.5, 0.5, 0.5, 1.0, // high need
                0.5,
            ));
        }

        let stats = system.distribute_energy(&mut requests);

        // Not all entities should get energy due to scarcity
        assert!(stats.entities_served < 100);
        assert!(stats.competition_intensity > 0.0);
    }

    #[test]
    fn test_energy_flow_system_get_energy_for_entity() {
        let mut system = EnergyFlowSystem::new(10000.0);

        let energy = system.get_energy_for_entity(1, 3, 0.8, 0.9, 0.7, 0.5, 0.8);

        assert!(energy > 0.0);
    }

    #[test]
    fn test_energy_flow_system_reset() {
        let mut system = EnergyFlowSystem::new(10000.0);

        // Distribute some energy
        let mut requests = vec![EnergyRequest::new(1, 3, 0.8, 0.9, 0.7, 0.5, 0.8)];
        system.distribute_energy(&mut requests);

        // Reset
        system.reset();

        // Check that pools are reset
        for pool in system.energy_pools.values() {
            assert_eq!(pool.allocated_energy, 0.0);
            assert_eq!(pool.competitor_count, 0);
        }
    }

    #[test]
    fn test_energy_flow_system_summary() {
        let system = EnergyFlowSystem::new(10000.0);
        let summary = system.summary();

        assert!(summary.contains("Energy Flow System Summary"));
        assert!(summary.contains("Input Energy: 10000.00"));
    }
}
