//! Social Memory Complex Evolution - 4th Density Integration
//!
//! From ROADMAP Phase 7.3: "Social Memory Complex for 4th Density"
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "A social memory complex is a collection of entities who have achieved
//! sufficient harmony to share a collective mind and memory."
//!
//! ## Overview
//!
//! When entities transition from 3rd to 4th density, they may form or join
//! a Social Memory Complex (SMC). An SMC is a collective consciousness where:
//!
//! 1. Individual entities retain their identity
//! 2. Memories and experiences are shared
//! 3. Consciousness operates at a group level
//! 4. Polarization is collective
//!
//! ## Harvest Mechanics
//!
//! For collective harvest:
//! - SMC average polarization must meet threshold (51% STO or 95% STS)
//! - Individual members must have compatible polarity
//! - Resonance threshold must be achieved

use crate::evolution_density_octave::density_octave::Density;
use crate::types::{Float, Polarity};
use std::collections::{HashMap, HashSet};

/// Entity identifier type for SMC operations
pub type SMCEntityId = u64;

/// Complex identifier type
pub type ComplexId = u64;

/// Minimum resonance threshold for SMC formation
pub const SMC_RESONANCE_THRESHOLD: Float = 0.7;

/// Minimum members for stable SMC
pub const SMC_MIN_MEMBERS: usize = 2;

/// SMC harvest thresholds
pub const SMC_STO_HARVEST_THRESHOLD: Float = 0.51;
pub const SMC_STS_HARVEST_THRESHOLD: Float = 0.95;

/// Collective memory shared by an SMC
#[derive(Debug, Clone)]
pub struct CollectiveMemoryData {
    /// Memory content
    pub content: Vec<Float>,

    /// Memory strength (0.0-1.0)
    pub strength: Float,

    /// Memory coherence (0.0-1.0)
    pub coherence: Float,

    /// Number of contributors
    pub contributor_count: usize,
}

impl Default for CollectiveMemoryData {
    fn default() -> Self {
        Self {
            content: Vec::new(),
            strength: 0.0,
            coherence: 0.0,
            contributor_count: 0,
        }
    }
}

/// Telepathic link between SMC members
#[derive(Debug, Clone)]
pub struct TelepathicLinkData {
    /// Source entity
    pub source: SMCEntityId,

    /// Target entity
    pub target: SMCEntityId,

    /// Link strength (0.0-1.0)
    pub strength: Float,

    /// Bandwidth (0.0-1.0)
    pub bandwidth: Float,
}

impl TelepathicLinkData {
    /// Create a new telepathic link
    pub fn new(source: SMCEntityId, target: SMCEntityId, strength: Float) -> Self {
        Self {
            source,
            target,
            strength: strength.clamp(0.0, 1.0),
            bandwidth: strength * 0.8,
        }
    }

    /// Check if link is functional
    pub fn is_functional(&self) -> bool {
        self.strength > 0.3 && self.bandwidth > 0.1
    }
}

/// Social Memory Complex with density awareness
#[derive(Debug, Clone)]
pub struct FourthDensitySMC {
    /// Unique identifier for this complex
    pub complex_id: ComplexId,

    /// Members of the SMC
    pub members: HashSet<SMCEntityId>,

    /// Current density of the SMC
    pub density: Density,

    /// Collective memory
    pub collective_memory: CollectiveMemoryData,

    /// Telepathic links between members
    pub telepathic_links: Vec<TelepathicLinkData>,

    /// Member polarization states
    pub member_polarities: HashMap<SMCEntityId, Polarity>,

    /// Member polarization intensities
    pub member_intensities: HashMap<SMCEntityId, Float>,

    /// Collective polarization (average)
    pub collective_polarization: Float,

    /// Collective polarity direction
    pub collective_polarity: Option<Polarity>,

    /// Whether harvested to 4th density
    pub harvested: bool,

    /// Harvest timestamp
    pub harvest_time: Option<u64>,

    /// SMC consciousness level
    pub consciousness_level: Float,

    /// Formation time
    pub formation_time: Float,
}

impl FourthDensitySMC {
    /// Create a new SMC
    pub fn new(complex_id: ComplexId, formation_time: Float) -> Self {
        Self {
            complex_id,
            members: HashSet::new(),
            density: Density::Third,
            collective_memory: CollectiveMemoryData::default(),
            telepathic_links: Vec::new(),
            member_polarities: HashMap::new(),
            member_intensities: HashMap::new(),
            collective_polarization: 0.0,
            collective_polarity: None,
            harvested: false,
            harvest_time: None,
            consciousness_level: 0.0,
            formation_time,
        }
    }

    /// Add a member to the SMC
    pub fn add_member(&mut self, entity_id: SMCEntityId, polarity: Polarity, intensity: Float) {
        self.members.insert(entity_id);
        self.member_polarities.insert(entity_id, polarity);
        self.member_intensities.insert(entity_id, intensity);
        self.update_collective_values();

        // Create telepathic links to existing members
        for &existing in self.members.iter() {
            if existing != entity_id {
                self.telepathic_links.push(TelepathicLinkData::new(
                    entity_id, existing, 0.8, // Initial link strength
                ));
            }
        }

        self.collective_memory.contributor_count = self.members.len();
    }

    /// Remove a member from the SMC
    pub fn remove_member(&mut self, entity_id: SMCEntityId) {
        self.members.remove(&entity_id);
        self.member_polarities.remove(&entity_id);
        self.member_intensities.remove(&entity_id);

        // Remove telepathic links involving this entity
        self.telepathic_links
            .retain(|link| link.source != entity_id && link.target != entity_id);

        self.update_collective_values();
        self.collective_memory.contributor_count = self.members.len();
    }

    /// Update collective polarization and polarity
    fn update_collective_values(&mut self) {
        if self.member_intensities.is_empty() {
            self.collective_polarization = 0.0;
            self.collective_polarity = None;
            return;
        }

        // Calculate average intensity
        let total: Float = self.member_intensities.values().sum();
        self.collective_polarization = total / self.member_intensities.len() as Float;

        // Determine collective polarity by majority
        let mut sto_count = 0;
        let mut sts_count = 0;
        let mut neutral_count = 0;

        for polarity in self.member_polarities.values() {
            match polarity {
                Polarity::STO | Polarity::ServiceToOthers => sto_count += 1,
                Polarity::STS | Polarity::ServiceToSelf => sts_count += 1,
                _ => neutral_count += 1,
            }
        }

        self.collective_polarity = if sto_count > sts_count && sto_count > neutral_count {
            Some(Polarity::STO)
        } else if sts_count > sto_count && sts_count > neutral_count {
            Some(Polarity::STS)
        } else {
            Some(Polarity::Neutral)
        };
    }

    /// Check harvest eligibility
    pub fn check_harvest_eligibility(&self) -> CollectiveHarvestEligibility {
        if self.density != Density::Third {
            return CollectiveHarvestEligibility::NotEligible {
                reason: "SMC not in 3rd density".to_string(),
            };
        }

        if self.members.len() < SMC_MIN_MEMBERS {
            return CollectiveHarvestEligibility::NotEligible {
                reason: format!(
                    "SMC needs at least {} members (has {})",
                    SMC_MIN_MEMBERS,
                    self.members.len()
                ),
            };
        }

        let polarity = match self.collective_polarity {
            Some(p) => p,
            None => {
                return CollectiveHarvestEligibility::NotEligible {
                    reason: "SMC has no collective polarity".to_string(),
                }
            }
        };

        match polarity {
            Polarity::STO | Polarity::ServiceToOthers => {
                if self.collective_polarization >= SMC_STO_HARVEST_THRESHOLD {
                    CollectiveHarvestEligibility::Eligible {
                        polarity: Polarity::STO,
                        intensity: self.collective_polarization,
                        target_density: Density::Fourth,
                    }
                } else {
                    CollectiveHarvestEligibility::NotEligible {
                        reason: format!(
                            "STO polarization {:.0}% below 51% threshold",
                            self.collective_polarization * 100.0
                        ),
                    }
                }
            }
            Polarity::STS | Polarity::ServiceToSelf => {
                if self.collective_polarization >= SMC_STS_HARVEST_THRESHOLD {
                    CollectiveHarvestEligibility::Eligible {
                        polarity: Polarity::STS,
                        intensity: self.collective_polarization,
                        target_density: Density::Fourth,
                    }
                } else {
                    CollectiveHarvestEligibility::NotEligible {
                        reason: format!(
                            "STS polarization {:.0}% below 95% threshold",
                            self.collective_polarization * 100.0
                        ),
                    }
                }
            }
            _ => CollectiveHarvestEligibility::NotEligible {
                reason: "SMC polarity is neutral - not harvestable".to_string(),
            },
        }
    }

    /// Execute collective harvest
    pub fn execute_harvest(&mut self, timestamp: u64) -> CollectiveHarvestResult {
        let eligibility = self.check_harvest_eligibility();

        match eligibility {
            CollectiveHarvestEligibility::Eligible { .. } => {
                self.density = Density::Fourth;
                self.harvested = true;
                self.harvest_time = Some(timestamp);
                self.consciousness_level = (self.consciousness_level + 0.3).min(1.0);
                self.collective_memory.coherence =
                    (self.collective_memory.coherence + 0.3).min(1.0);

                CollectiveHarvestResult::Success {
                    complex_id: self.complex_id,
                    member_count: self.members.len(),
                    to_density: Density::Fourth,
                    timestamp,
                }
            }
            CollectiveHarvestEligibility::NotEligible { reason } => {
                CollectiveHarvestResult::Failed { reason }
            }
        }
    }

    /// Access collective memory
    pub fn access_memory(&self, entity_id: SMCEntityId, query: &str) -> Option<String> {
        if !self.members.contains(&entity_id) {
            return None;
        }

        if self.collective_memory.coherence > 0.5 {
            Some(format!(
                "Collective memory: {} (coherence: {:.0}%)",
                query,
                self.collective_memory.coherence * 100.0
            ))
        } else {
            None
        }
    }

    /// Check if SMC is stable
    pub fn is_stable(&self) -> bool {
        self.members.len() >= SMC_MIN_MEMBERS
    }

    /// Get member count
    pub fn member_count(&self) -> usize {
        self.members.len()
    }

    /// Check if in 4th density
    pub fn is_fourth_density(&self) -> bool {
        matches!(self.density, Density::Fourth)
    }
}

/// Collective harvest eligibility status
#[derive(Debug, Clone)]
pub enum CollectiveHarvestEligibility {
    /// Eligible for harvest
    Eligible {
        polarity: Polarity,
        intensity: Float,
        target_density: Density,
    },

    /// Not eligible
    NotEligible { reason: String },
}

/// Result of collective harvest
#[derive(Debug, Clone)]
pub enum CollectiveHarvestResult {
    /// Success
    Success {
        complex_id: ComplexId,
        member_count: usize,
        to_density: Density,
        timestamp: u64,
    },

    /// Failed
    Failed { reason: String },
}

/// SMC Formation System
#[derive(Debug, Clone)]
pub struct SMCFormationSystem {
    /// All SMCs
    pub complexes: HashMap<ComplexId, FourthDensitySMC>,

    /// Entity to SMC mapping
    pub entity_smc_map: HashMap<SMCEntityId, ComplexId>,

    /// Formation history
    pub formation_history: Vec<FormationEvent>,

    /// Next SMC ID
    next_smc_id: ComplexId,
}

/// Formation event record
#[derive(Debug, Clone)]
pub struct FormationEvent {
    pub complex_id: ComplexId,
    pub members: Vec<SMCEntityId>,
    pub timestamp: u64,
    pub success: bool,
    pub reason: Option<String>,
}

impl SMCFormationSystem {
    /// Create a new formation system
    pub fn new() -> Self {
        Self {
            complexes: HashMap::new(),
            entity_smc_map: HashMap::new(),
            formation_history: Vec::new(),
            next_smc_id: 1,
        }
    }

    /// Attempt to form a new SMC
    pub fn attempt_formation(
        &mut self,
        candidates: &[SMCEntityId],
        polarities: &HashMap<SMCEntityId, Polarity>,
        intensities: &HashMap<SMCEntityId, Float>,
        consciousness_levels: &HashMap<SMCEntityId, Float>,
        timestamp: u64,
    ) -> SMCFormationResult {
        // Check minimum members
        if candidates.len() < SMC_MIN_MEMBERS {
            return SMCFormationResult::Failed {
                reason: format!("Need at least {} candidates", SMC_MIN_MEMBERS),
            };
        }

        // Check individual eligibility
        for &entity_id in candidates {
            let polarity = polarities
                .get(&entity_id)
                .copied()
                .unwrap_or(Polarity::Neutral);
            let intensity = intensities.get(&entity_id).copied().unwrap_or(0.0);

            let eligible = match polarity {
                Polarity::STO | Polarity::ServiceToOthers => intensity >= SMC_STO_HARVEST_THRESHOLD,
                Polarity::STS | Polarity::ServiceToSelf => intensity >= SMC_STS_HARVEST_THRESHOLD,
                _ => false,
            };

            if !eligible {
                return SMCFormationResult::Failed {
                    reason: format!("Entity {} not individually harvest-eligible", entity_id),
                };
            }
        }

        // Check polarity compatibility
        let first_polarity = polarities.get(&candidates[0]).copied();
        let compatible = candidates.iter().all(|&id| {
            let p = polarities.get(&id).copied();
            Self::polarities_compatible(first_polarity, p)
        });

        if !compatible {
            return SMCFormationResult::Failed {
                reason: "Candidates have incompatible polarities".to_string(),
            };
        }

        // Calculate resonance
        let resonance = self.calculate_resonance(candidates, intensities, consciousness_levels);

        if resonance < SMC_RESONANCE_THRESHOLD {
            return SMCFormationResult::Failed {
                reason: format!(
                    "Resonance {:.0}% below {:.0}% threshold",
                    resonance * 100.0,
                    SMC_RESONANCE_THRESHOLD * 100.0
                ),
            };
        }

        // Create SMC
        let complex_id = self.next_smc_id;
        self.next_smc_id += 1;

        let mut smc = FourthDensitySMC::new(complex_id, timestamp as Float);

        for &entity_id in candidates {
            let polarity = polarities
                .get(&entity_id)
                .copied()
                .unwrap_or(Polarity::Neutral);
            let intensity = intensities.get(&entity_id).copied().unwrap_or(0.0);
            smc.add_member(entity_id, polarity, intensity);
            self.entity_smc_map.insert(entity_id, complex_id);
        }

        smc.consciousness_level =
            consciousness_levels.values().sum::<Float>() / consciousness_levels.len() as Float;

        self.complexes.insert(complex_id, smc);

        self.formation_history.push(FormationEvent {
            complex_id,
            members: candidates.to_vec(),
            timestamp,
            success: true,
            reason: None,
        });

        SMCFormationResult::Success { complex_id }
    }

    /// Check if two polarities are compatible
    fn polarities_compatible(a: Option<Polarity>, b: Option<Polarity>) -> bool {
        match (a, b) {
            (Some(Polarity::STO), Some(Polarity::STO)) => true,
            (Some(Polarity::ServiceToOthers), Some(Polarity::ServiceToOthers)) => true,
            (Some(Polarity::STO), Some(Polarity::ServiceToOthers)) => true,
            (Some(Polarity::ServiceToOthers), Some(Polarity::STO)) => true,
            (Some(Polarity::STS), Some(Polarity::STS)) => true,
            (Some(Polarity::ServiceToSelf), Some(Polarity::ServiceToSelf)) => true,
            (Some(Polarity::STS), Some(Polarity::ServiceToSelf)) => true,
            (Some(Polarity::ServiceToSelf), Some(Polarity::STS)) => true,
            _ => false,
        }
    }

    /// Calculate group resonance
    fn calculate_resonance(
        &self,
        candidates: &[SMCEntityId],
        intensities: &HashMap<SMCEntityId, Float>,
        consciousness_levels: &HashMap<SMCEntityId, Float>,
    ) -> Float {
        if candidates.len() < 2 {
            return 0.0;
        }

        let mut total_resonance = 0.0;
        let mut pair_count = 0;

        for i in 0..candidates.len() {
            for j in (i + 1)..candidates.len() {
                let intensity_a = intensities.get(&candidates[i]).copied().unwrap_or(0.0);
                let intensity_b = intensities.get(&candidates[j]).copied().unwrap_or(0.0);
                let consciousness_a = consciousness_levels
                    .get(&candidates[i])
                    .copied()
                    .unwrap_or(0.0);
                let consciousness_b = consciousness_levels
                    .get(&candidates[j])
                    .copied()
                    .unwrap_or(0.0);

                let intensity_match = 1.0 - (intensity_a - intensity_b).abs();
                let consciousness_match = 1.0 - (consciousness_a - consciousness_b).abs();

                total_resonance += (intensity_match + consciousness_match) / 2.0;
                pair_count += 1;
            }
        }

        if pair_count == 0 {
            0.0
        } else {
            total_resonance / pair_count as Float
        }
    }

    /// Get SMC for an entity
    pub fn get_smc_for_entity(&self, entity_id: SMCEntityId) -> Option<&FourthDensitySMC> {
        self.entity_smc_map
            .get(&entity_id)
            .and_then(|&complex_id| self.complexes.get(&complex_id))
    }

    /// Process harvests for all SMCs
    pub fn process_collective_harvests(&mut self, timestamp: u64) -> Vec<CollectiveHarvestResult> {
        let mut results = Vec::new();

        for smc in self.complexes.values_mut() {
            if !smc.harvested {
                results.push(smc.execute_harvest(timestamp));
            }
        }

        results
    }

    /// Get statistics
    pub fn statistics(&self) -> SMCStatistics {
        let total_smcs = self.complexes.len();
        let harvested_smcs = self.complexes.values().filter(|s| s.harvested).count();
        let total_members: usize = self.complexes.values().map(|s| s.member_count()).sum();

        SMCStatistics {
            total_smcs,
            harvested_smcs,
            third_density_smcs: total_smcs - harvested_smcs,
            total_members,
            average_size: if total_smcs > 0 {
                total_members as Float / total_smcs as Float
            } else {
                0.0
            },
        }
    }
}

impl Default for SMCFormationSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// SMC formation result
#[derive(Debug, Clone)]
pub enum SMCFormationResult {
    Success { complex_id: ComplexId },
    Failed { reason: String },
}

/// SMC statistics
#[derive(Debug, Clone)]
pub struct SMCStatistics {
    pub total_smcs: usize,
    pub harvested_smcs: usize,
    pub third_density_smcs: usize,
    pub total_members: usize,
    pub average_size: Float,
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smc_creation() {
        let smc = FourthDensitySMC::new(1, 0.0);

        assert_eq!(smc.density, Density::Third);
        assert!(!smc.harvested);
        assert!(smc.members.is_empty());
    }

    #[test]
    fn test_add_member() {
        let mut smc = FourthDensitySMC::new(1, 0.0);

        smc.add_member(1, Polarity::STO, 0.6);

        assert_eq!(smc.member_count(), 1);
        assert_eq!(smc.collective_polarity, Some(Polarity::STO));
        assert!((smc.collective_polarization - 0.6).abs() < 0.001);
    }

    #[test]
    fn test_collective_polarity_majority() {
        let mut smc = FourthDensitySMC::new(1, 0.0);

        smc.add_member(1, Polarity::STO, 0.6);
        smc.add_member(2, Polarity::STO, 0.7);
        smc.add_member(3, Polarity::STS, 0.8);

        assert_eq!(smc.collective_polarity, Some(Polarity::STO));
    }

    #[test]
    fn test_harvest_eligibility_sto_success() {
        let mut smc = FourthDensitySMC::new(1, 0.0);

        smc.add_member(1, Polarity::STO, 0.6);
        smc.add_member(2, Polarity::STO, 0.6);

        let eligibility = smc.check_harvest_eligibility();

        match eligibility {
            CollectiveHarvestEligibility::Eligible { polarity, .. } => {
                assert_eq!(polarity, Polarity::STO);
            }
            _ => panic!("Expected eligibility"),
        }
    }

    #[test]
    fn test_harvest_eligibility_sts_success() {
        let mut smc = FourthDensitySMC::new(1, 0.0);

        smc.add_member(1, Polarity::STS, 0.96);
        smc.add_member(2, Polarity::STS, 0.96);

        let eligibility = smc.check_harvest_eligibility();

        match eligibility {
            CollectiveHarvestEligibility::Eligible { polarity, .. } => {
                assert_eq!(polarity, Polarity::STS);
            }
            _ => panic!("Expected eligibility"),
        }
    }

    #[test]
    fn test_harvest_eligibility_insufficient() {
        let mut smc = FourthDensitySMC::new(1, 0.0);

        smc.add_member(1, Polarity::STO, 0.4);
        smc.add_member(2, Polarity::STO, 0.4);

        let eligibility = smc.check_harvest_eligibility();

        match eligibility {
            CollectiveHarvestEligibility::NotEligible { reason } => {
                assert!(reason.contains("below 51%"));
            }
            _ => panic!("Expected not eligible"),
        }
    }

    #[test]
    fn test_execute_harvest_success() {
        let mut smc = FourthDensitySMC::new(1, 0.0);

        smc.add_member(1, Polarity::STO, 0.6);
        smc.add_member(2, Polarity::STO, 0.6);

        let result = smc.execute_harvest(100);

        match result {
            CollectiveHarvestResult::Success { to_density, .. } => {
                assert_eq!(to_density, Density::Fourth);
                assert!(smc.harvested);
                assert_eq!(smc.harvest_time, Some(100));
            }
            _ => panic!("Expected success"),
        }
    }

    #[test]
    fn test_formation_system() {
        let mut system = SMCFormationSystem::new();

        let candidates = vec![1, 2];
        let polarities = vec![(1, Polarity::STO), (2, Polarity::STO)]
            .into_iter()
            .collect();
        let intensities = vec![(1, 0.6), (2, 0.7)].into_iter().collect();
        let consciousness = vec![(1, 0.7), (2, 0.7)].into_iter().collect();

        let result =
            system.attempt_formation(&candidates, &polarities, &intensities, &consciousness, 0);

        match result {
            SMCFormationResult::Success { complex_id } => {
                assert!(system.complexes.contains_key(&complex_id));
            }
            _ => panic!("Expected success"),
        }
    }

    #[test]
    fn test_formation_incompatible_polarities() {
        let mut system = SMCFormationSystem::new();

        let candidates = vec![1, 2];
        let polarities = vec![(1, Polarity::STO), (2, Polarity::STS)]
            .into_iter()
            .collect();
        let intensities = vec![(1, 0.6), (2, 0.96)].into_iter().collect();
        let consciousness = vec![(1, 0.7), (2, 0.7)].into_iter().collect();

        let result =
            system.attempt_formation(&candidates, &polarities, &intensities, &consciousness, 0);

        match result {
            SMCFormationResult::Failed { reason } => {
                assert!(reason.contains("incompatible"));
            }
            _ => panic!("Expected failure"),
        }
    }

    #[test]
    fn test_access_memory() {
        let mut smc = FourthDensitySMC::new(1, 0.0);
        smc.add_member(1, Polarity::STO, 0.6);
        smc.collective_memory.coherence = 0.8;

        let result = smc.access_memory(1, "test query");
        assert!(result.is_some());

        let result = smc.access_memory(999, "test query");
        assert!(result.is_none());
    }

    #[test]
    fn test_telepathic_link() {
        let link = TelepathicLinkData::new(1, 2, 0.8);

        assert!(link.is_functional());

        let weak_link = TelepathicLinkData::new(1, 2, 0.2);
        assert!(!weak_link.is_functional());
    }

    #[test]
    fn test_statistics() {
        let mut system = SMCFormationSystem::new();

        // Create two SMCs
        for i in 0..2 {
            let candidates = vec![i * 2 + 1, i * 2 + 2];
            let polarities: HashMap<_, _> =
                candidates.iter().map(|&id| (id, Polarity::STO)).collect();
            let intensities: HashMap<_, _> = candidates.iter().map(|&id| (id, 0.6)).collect();
            let consciousness: HashMap<_, _> = candidates.iter().map(|&id| (id, 0.7)).collect();

            let _ =
                system.attempt_formation(&candidates, &polarities, &intensities, &consciousness, 0);
        }

        let stats = system.statistics();
        assert_eq!(stats.total_smcs, 2);
        assert_eq!(stats.total_members, 4);
    }

    #[test]
    fn test_process_collective_harvests() {
        let mut system = SMCFormationSystem::new();

        let candidates = vec![1, 2];
        let polarities: HashMap<_, _> = candidates.iter().map(|&id| (id, Polarity::STO)).collect();
        let intensities: HashMap<_, _> = candidates.iter().map(|&id| (id, 0.6)).collect();
        let consciousness: HashMap<_, _> = candidates.iter().map(|&id| (id, 0.7)).collect();

        let _ = system.attempt_formation(&candidates, &polarities, &intensities, &consciousness, 0);

        let results = system.process_collective_harvests(100);
        assert_eq!(results.len(), 1);

        match &results[0] {
            CollectiveHarvestResult::Success { .. } => {}
            _ => panic!("Expected success"),
        }
    }
}
