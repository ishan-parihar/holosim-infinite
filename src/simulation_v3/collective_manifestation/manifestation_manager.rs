//! Manifestation Manager System
//!
//! From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md Phase 6, Week 89-92:
//! "Collective Manifestation (Building) - multiple entities contribute resonance
//! to create structures"
//!
//! This module implements the manifestation manager that:
//! - Manages active building projects
//! - Tracks collective resonance contributions
//! - Updates build progress based on resonance accumulation
//! - Converts completed projects to structures
//! - Validates requirements before starting projects
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Building as collective manifestation - structures emerge from collective resonance"

use crate::entity_layer7::layer7::{EntityId, SubSubLogos};
use crate::simulation_v3::collective_manifestation::collective_resonance::{
    CollectiveResonanceCalculator, CollectiveResonanceResult,
};
use crate::simulation_v3::collective_manifestation::holographic_structure::{
    HolographicStructure, StructureType,
};
use crate::types::Float;
use std::collections::HashMap;

/// Errors that can occur during manifestation operations
///
/// From AGENTS.md: Error Handling Guidelines
#[derive(Debug, Clone, PartialEq)]
pub enum ManifestationError {
    /// Project with the given ID not found
    ProjectNotFound(u64),
    /// Insufficient resonance (required, current)
    InsufficientResonance(Float, Float),
    /// Invalid contributors list (e.g., empty or contains duplicates)
    InvalidContributors(Vec<EntityId>),
    /// Structure with the given ID already exists
    StructureAlreadyExists(u64),
    /// Project is already complete
    ProjectAlreadyComplete(u64),
}

impl std::fmt::Display for ManifestationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ManifestationError::ProjectNotFound(id) => {
                write!(f, "Project {} not found", id)
            }
            ManifestationError::InsufficientResonance(required, current) => {
                write!(
                    f,
                    "Insufficient resonance: required {}, current {}",
                    required, current
                )
            }
            ManifestationError::InvalidContributors(ids) => {
                write!(f, "Invalid contributors: {:?}", ids)
            }
            ManifestationError::StructureAlreadyExists(id) => {
                write!(f, "Structure {} already exists", id)
            }
            ManifestationError::ProjectAlreadyComplete(id) => {
                write!(f, "Project {} is already complete", id)
            }
        }
    }
}

/// An active building project
///
/// Represents a collective manifestation effort where multiple entities
/// contribute their resonance to create a holographic structure.
///
/// From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md Phase 6:
/// "Building projects track collective resonance contributions over time"
#[derive(Debug, Clone)]
pub struct ManifestationProject {
    /// Unique identifier for this project
    pub project_id: u64,
    /// The structure being manifested
    pub structure: HolographicStructure,
    /// Entity IDs contributing to this project
    pub contributing_entities: Vec<EntityId>,
    /// Current collective resonance result
    pub collective_resonance: CollectiveResonanceResult,
    /// Project start time (simulation time)
    pub start_time: Float,
    /// Completion progress (0.0 to 1.0)
    pub completion_progress: Float,
}

impl ManifestationProject {
    /// Create a new manifestation project
    ///
    /// # Arguments
    /// * `project_id` - Unique identifier for the project
    /// * `structure` - The structure being manifested
    /// * `contributing_entities` - Entity IDs contributing to this project
    /// * `collective_resonance` - Initial collective resonance result
    /// * `start_time` - Project start time
    ///
    /// # Returns
    /// * New `ManifestationProject` instance
    pub fn new(
        project_id: u64,
        structure: HolographicStructure,
        contributing_entities: Vec<EntityId>,
        collective_resonance: CollectiveResonanceResult,
        start_time: Float,
    ) -> Self {
        Self {
            project_id,
            structure,
            contributing_entities,
            collective_resonance,
            start_time,
            completion_progress: 0.0,
        }
    }

    /// Check if project is complete
    ///
    /// # Returns
    /// * `true` if completion progress >= 1.0, `false` otherwise
    pub fn is_complete(&self) -> bool {
        self.completion_progress >= 1.0
    }

    /// Get the project duration
    ///
    /// # Arguments
    /// * `current_time` - Current simulation time
    ///
    /// # Returns
    /// * Duration since project start
    pub fn duration(&self, current_time: Float) -> Float {
        (current_time - self.start_time).max(0.0)
    }

    /// Get the number of contributing entities
    pub fn contributor_count(&self) -> usize {
        self.contributing_entities.len()
    }
}

/// Manages collective manifestation of structures
///
/// The manifestation manager coordinates building projects, tracking resonance
/// contributions from entities and managing the transition from active projects
/// to completed structures.
///
/// From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md Phase 6:
/// "The manifestation manager oversees all collective building efforts"
pub struct ManifestationManager {
    /// Active building projects
    active_projects: HashMap<u64, ManifestationProject>,
    /// Completed structures
    completed_structures: HashMap<u64, HolographicStructure>,
    /// Next project ID to assign
    next_project_id: u64,
}

impl ManifestationManager {
    /// Create a new manifestation manager
    ///
    /// # Returns
    /// * New `ManifestationManager` instance with empty project lists
    pub fn new() -> Self {
        Self {
            active_projects: HashMap::new(),
            completed_structures: HashMap::new(),
            next_project_id: 1,
        }
    }

    /// Create a new building project
    ///
    /// Validates resonance requirements and creates a new project for collective
    /// manifestation of a structure.
    ///
    /// # Arguments
    /// * `entities` - Slice of entity references contributing to the project
    /// * `structure_type` - Type of structure to manifest
    /// * `position` - 3D position for structure manifestation
    /// * `current_time` - Current simulation time
    ///
    /// # Returns
    /// * `Ok(project_id)` - ID of the created project
    /// * `Err(ManifestationError)` - If project creation fails
    ///
    /// From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md Phase 6:
    /// "Projects are created only when resonance requirements are met"
    pub fn create_project(
        &mut self,
        entities: &[&SubSubLogos],
        structure_type: StructureType,
        position: [Float; 3],
        current_time: Float,
    ) -> Result<u64, ManifestationError> {
        // Validate contributors
        if entities.is_empty() {
            return Err(ManifestationError::InvalidContributors(vec![]));
        }

        // Check for duplicate entities
        let entity_ids: Vec<EntityId> = entities.iter().map(|e| e.entity_id.clone()).collect();
        let unique_ids: std::collections::HashSet<_> = entity_ids.iter().collect();
        if unique_ids.len() != entity_ids.len() {
            return Err(ManifestationError::InvalidContributors(entity_ids));
        }

        // Compute collective resonance
        let collective_resonance = CollectiveResonanceCalculator::compute_from_entities(entities);

        // Create holographic structure from collective resonance
        let project_id = self.next_project_id;
        self.next_project_id += 1;

        let mut structure = HolographicStructure::from_collective_resonance(
            project_id,
            structure_type,
            &collective_resonance,
            position,
        );

        // Add contributors to the structure
        for entity in entities {
            structure.add_contributor(entity.entity_id.uuid.parse().unwrap_or(0));
        }

        // Check if structure with this ID already exists
        if self.completed_structures.contains_key(&project_id) {
            return Err(ManifestationError::StructureAlreadyExists(project_id));
        }

        // Create manifestation project
        let project = ManifestationProject::new(
            project_id,
            structure,
            entity_ids,
            collective_resonance,
            current_time,
        );

        // Add project to active projects
        self.active_projects.insert(project_id, project);

        Ok(project_id)
    }

    /// Contribute resonance from an entity to a project
    ///
    /// Adds resonance contribution from an entity to the specified project.
    /// Updates the collective resonance and build progress.
    ///
    /// # Arguments
    /// * `project_id` - ID of the project to contribute to
    /// * `_entity` - Entity contributing resonance (reference for future validation)
    /// * `resonance_amount` - Amount of resonance to contribute
    ///
    /// # Returns
    /// * `Ok(())` - If contribution was successful
    /// * `Err(ManifestationError)` - If contribution fails
    ///
    /// From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md Phase 6:
    /// "Entities contribute resonance to accelerate building progress"
    pub fn contribute_resonance(
        &mut self,
        project_id: u64,
        _entity: &SubSubLogos,
        resonance_amount: Float,
    ) -> Result<(), ManifestationError> {
        // Get the project
        let project = self
            .active_projects
            .get_mut(&project_id)
            .ok_or(ManifestationError::ProjectNotFound(project_id))?;

        // Check if project is already complete
        if project.is_complete() {
            return Err(ManifestationError::ProjectAlreadyComplete(project_id));
        }

        // Update build progress
        project.structure.update_build_progress(resonance_amount);
        project.completion_progress = project.structure.build_progress;

        // Update structure stability based on current coherence
        project
            .structure
            .update_stability(project.collective_resonance.coherence);

        Ok(())
    }

    /// Update a project's progress
    ///
    /// Recomputes collective resonance and updates build progress based on
    /// the current state of contributing entities.
    ///
    /// # Arguments
    /// * `project_id` - ID of the project to update
    /// * `entities` - Slice of entity references (must include all contributors)
    ///
    /// # Returns
    /// * `Ok(())` - If update was successful
    /// * `Err(ManifestationError)` - If update fails
    ///
    /// From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md Phase 6:
    /// "Project progress is updated periodically as entities evolve"
    pub fn update_project(
        &mut self,
        project_id: u64,
        entities: &[&SubSubLogos],
    ) -> Result<(), ManifestationError> {
        // Get the project
        let project = self
            .active_projects
            .get_mut(&project_id)
            .ok_or(ManifestationError::ProjectNotFound(project_id))?;

        // Check if project is already complete
        if project.is_complete() {
            return Err(ManifestationError::ProjectAlreadyComplete(project_id));
        }

        // Recompute collective resonance
        let collective_resonance = CollectiveResonanceCalculator::compute_from_entities(entities);
        project.collective_resonance = collective_resonance.clone();

        // Update structure stability
        project
            .structure
            .update_stability(collective_resonance.coherence);

        // Add resonance contribution based on collective strength
        let resonance_contribution = collective_resonance.resonance_strength;
        project
            .structure
            .update_build_progress(resonance_contribution);
        project.completion_progress = project.structure.build_progress;

        // Check if project is complete
        if project.is_complete() {
            self.complete_project(project_id)?;
        }

        Ok(())
    }

    /// Get an active project by ID
    ///
    /// # Arguments
    /// * `project_id` - ID of the project to retrieve
    ///
    /// # Returns
    /// * `Ok(&ManifestationProject)` - Reference to the project
    /// * `Err(ManifestationError)` - If project not found
    pub fn get_project(
        &self,
        project_id: u64,
    ) -> Result<&ManifestationProject, ManifestationError> {
        self.active_projects
            .get(&project_id)
            .ok_or(ManifestationError::ProjectNotFound(project_id))
    }

    /// Get a completed structure by ID
    ///
    /// # Arguments
    /// * `structure_id` - ID of the structure to retrieve
    ///
    /// # Returns
    /// * `Ok(&HolographicStructure)` - Reference to the structure
    /// * `Err(ManifestationError)` - If structure not found
    pub fn get_completed_structure(
        &self,
        structure_id: u64,
    ) -> Result<&HolographicStructure, ManifestationError> {
        self.completed_structures
            .get(&structure_id)
            .ok_or(ManifestationError::ProjectNotFound(structure_id))
    }

    /// List all active projects
    ///
    /// # Returns
    /// * Vector of active project IDs
    pub fn list_active_projects(&self) -> Vec<u64> {
        let mut project_ids: Vec<u64> = self.active_projects.keys().copied().collect();
        project_ids.sort();
        project_ids
    }

    /// List all completed structures
    ///
    /// # Returns
    /// * Vector of completed structure IDs
    pub fn list_completed_structures(&self) -> Vec<u64> {
        let mut structure_ids: Vec<u64> = self.completed_structures.keys().copied().collect();
        structure_ids.sort();
        structure_ids
    }

    /// Check if requirements are met for a project
    ///
    /// Validates that the current collective resonance meets the required
    /// threshold for the project's structure.
    ///
    /// # Arguments
    /// * `project_id` - ID of the project to check
    ///
    /// # Returns
    /// * `Ok(true)` - Requirements are met
    /// * `Ok(false)` - Requirements not met but project exists
    /// * `Err(ManifestationError)` - If project not found
    ///
    /// From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md Phase 6:
    /// "Requirements are checked before significant resonance accumulation"
    pub fn check_requirements(&self, project_id: u64) -> Result<bool, ManifestationError> {
        let project = self
            .active_projects
            .get(&project_id)
            .ok_or(ManifestationError::ProjectNotFound(project_id))?;

        // Check if resonance meets threshold
        let current_resonance = project.structure.current_resonance;
        let required_resonance = project.structure.required_resonance;

        Ok(current_resonance >= required_resonance * 0.1) // At least 10% progress
    }

    /// Cancel an active project
    ///
    /// Removes a project from the active list. The project's progress is lost.
    ///
    /// # Arguments
    /// * `project_id` - ID of the project to cancel
    ///
    /// # Returns
    /// * `Ok(())` - If cancellation was successful
    /// * `Err(ManifestationError)` - If project not found
    ///
    /// From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md Phase 6:
    /// "Projects can be cancelled if they become unfeasible"
    pub fn cancel_project(&mut self, project_id: u64) -> Result<(), ManifestationError> {
        self.active_projects
            .remove(&project_id)
            .ok_or(ManifestationError::ProjectNotFound(project_id))?;
        Ok(())
    }

    /// Complete a project and move it to completed structures
    ///
    /// Internal method called when a project reaches 100% completion.
    ///
    /// # Arguments
    /// * `project_id` - ID of the project to complete
    ///
    /// # Returns
    /// * `Ok(())` - If completion was successful
    /// * `Err(ManifestationError)` - If project not found or already complete
    fn complete_project(&mut self, project_id: u64) -> Result<(), ManifestationError> {
        let project = self
            .active_projects
            .remove(&project_id)
            .ok_or(ManifestationError::ProjectNotFound(project_id))?;

        // Ensure build progress is exactly 1.0
        let mut structure = project.structure;
        structure.build_progress = 1.0;

        // Add to completed structures
        self.completed_structures.insert(project_id, structure);

        Ok(())
    }

    /// Get the number of active projects
    pub fn active_project_count(&self) -> usize {
        self.active_projects.len()
    }

    /// Get the number of completed structures
    pub fn completed_structure_count(&self) -> usize {
        self.completed_structures.len()
    }

    /// Check if a project exists
    pub fn has_project(&self, project_id: u64) -> bool {
        self.active_projects.contains_key(&project_id)
    }

    /// Check if a structure exists
    pub fn has_structure(&self, structure_id: u64) -> bool {
        self.completed_structures.contains_key(&structure_id)
    }
}

impl Default for ManifestationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to create a test entity with minimal setup
    /// Note: This creates a simplified entity for testing purposes only
    fn create_test_entity(
        _id: u64,
        space_time_access: Float,
        time_space_access: Float,
    ) -> SubSubLogos {
        // Create a minimal entity using the spectrum values directly
        // We create the entity with default values and then set the spectrum access
        let mut entity = SubSubLogos::create_test_entity();

        // Set the spectrum access values for testing
        entity.spectrum_access.space_time_access = space_time_access;
        entity.spectrum_access.time_space_access = time_space_access;

        entity
    }

    #[test]
    fn test_manager_creation() {
        let manager = ManifestationManager::new();

        assert_eq!(manager.active_project_count(), 0);
        assert_eq!(manager.completed_structure_count(), 0);
        assert!(!manager.has_project(1));
        assert!(!manager.has_structure(1));
    }

    #[test]
    fn test_manager_default() {
        let manager = ManifestationManager::default();

        assert_eq!(manager.active_project_count(), 0);
        assert_eq!(manager.completed_structure_count(), 0);
    }

    #[test]
    fn test_project_creation() {
        let mut manager = ManifestationManager::new();

        let entity1 = create_test_entity(1, 0.6, 0.4);
        let entity2 = create_test_entity(2, 0.55, 0.45);
        let entity3 = create_test_entity(3, 0.65, 0.35);

        let entities = vec![&entity1, &entity2, &entity3];
        let project_id = manager
            .create_project(&entities, StructureType::Temple, [0.0, 0.0, 0.0], 0.0)
            .expect("Failed to create project");

        assert_eq!(project_id, 1);
        assert_eq!(manager.active_project_count(), 1);
        assert!(manager.has_project(project_id));
        assert!(!manager.has_structure(project_id));
    }

    #[test]
    fn test_project_creation_empty_contributors() {
        let mut manager = ManifestationManager::new();

        let entities: Vec<&SubSubLogos> = vec![];
        let result = manager.create_project(&entities, StructureType::Temple, [0.0, 0.0, 0.0], 0.0);

        assert_eq!(result, Err(ManifestationError::InvalidContributors(vec![])));
        assert_eq!(manager.active_project_count(), 0);
    }

    #[test]
    fn test_project_creation_duplicate_contributors() {
        let mut manager = ManifestationManager::new();

        let entity1 = create_test_entity(1, 0.6, 0.4);

        // Add same entity twice (different references but same entity)
        let entities = vec![&entity1, &entity1];
        let result = manager.create_project(&entities, StructureType::Temple, [0.0, 0.0, 0.0], 0.0);

        assert!(matches!(
            result,
            Err(ManifestationError::InvalidContributors(_))
        ));
        assert_eq!(manager.active_project_count(), 0);
    }

    #[test]
    fn test_resonance_contribution() {
        let mut manager = ManifestationManager::new();

        let entity1 = create_test_entity(1, 0.6, 0.4);
        let entity2 = create_test_entity(2, 0.55, 0.45);

        let entities = vec![&entity1, &entity2];
        let project_id = manager
            .create_project(&entities, StructureType::Temple, [0.0, 0.0, 0.0], 0.0)
            .expect("Failed to create project");

        let project = manager.get_project(project_id).unwrap();
        let initial_progress = project.completion_progress;

        // Contribute resonance
        manager
            .contribute_resonance(project_id, &entity1, 5.0)
            .expect("Failed to contribute resonance");

        let project = manager.get_project(project_id).unwrap();
        assert!(project.completion_progress > initial_progress);
    }

    #[test]
    fn test_resonance_contribution_nonexistent_project() {
        let mut manager = ManifestationManager::new();

        let entity1 = create_test_entity(1, 0.6, 0.4);
        let result = manager.contribute_resonance(999, &entity1, 5.0);

        assert_eq!(result, Err(ManifestationError::ProjectNotFound(999)));
    }

    #[test]
    fn test_progress_update() {
        let mut manager = ManifestationManager::new();

        let entity1 = create_test_entity(1, 0.6, 0.4);
        let entity2 = create_test_entity(2, 0.55, 0.45);

        let entities = vec![&entity1, &entity2];
        let project_id = manager
            .create_project(&entities, StructureType::Temple, [0.0, 0.0, 0.0], 0.0)
            .expect("Failed to create project");

        let project = manager.get_project(project_id).unwrap();
        let initial_progress = project.completion_progress;

        // Update project
        manager
            .update_project(project_id, &entities)
            .expect("Failed to update project");

        let project = manager.get_project(project_id).unwrap();
        assert!(project.completion_progress >= initial_progress);
    }

    #[test]
    fn test_update_nonexistent_project() {
        let mut manager = ManifestationManager::new();

        let entity1 = create_test_entity(1, 0.6, 0.4);
        let entities = vec![&entity1];
        let result = manager.update_project(999, &entities);

        assert_eq!(result, Err(ManifestationError::ProjectNotFound(999)));
    }

    #[test]
    fn test_completion() {
        let mut manager = ManifestationManager::new();

        let entity1 = create_test_entity(1, 0.6, 0.4);
        let entity2 = create_test_entity(2, 0.55, 0.45);

        let entities = vec![&entity1, &entity2];
        let project_id = manager
            .create_project(&entities, StructureType::Temple, [0.0, 0.0, 0.0], 0.0)
            .expect("Failed to create project");

        // Get the project to determine required resonance
        let project = manager.get_project(project_id).unwrap();
        let required_resonance = project.structure.required_resonance;

        // Contribute enough resonance to complete
        manager
            .contribute_resonance(project_id, &entity1, required_resonance * 0.6)
            .expect("Failed to contribute resonance");

        manager
            .contribute_resonance(project_id, &entity2, required_resonance * 0.5)
            .expect("Failed to contribute resonance");

        // Check that project moved to completed structures
        assert_eq!(manager.active_project_count(), 0);
        assert_eq!(manager.completed_structure_count(), 1);
        assert!(!manager.has_project(project_id));
        assert!(manager.has_structure(project_id));

        // Verify completed structure
        let structure = manager.get_completed_structure(project_id).unwrap();
        assert!(structure.is_complete());
        assert_eq!(structure.build_progress, 1.0);
    }

    #[test]
    fn test_contribute_to_completed_project() {
        let mut manager = ManifestationManager::new();

        let entity1 = create_test_entity(1, 0.6, 0.4);
        let entity2 = create_test_entity(2, 0.55, 0.45);

        let entities = vec![&entity1, &entity2];
        let project_id = manager
            .create_project(&entities, StructureType::Temple, [0.0, 0.0, 0.0], 0.0)
            .expect("Failed to create project");

        // Get required resonance and complete project
        let project = manager.get_project(project_id).unwrap();
        let required_resonance = project.structure.required_resonance;

        manager
            .contribute_resonance(project_id, &entity1, required_resonance * 2.0)
            .expect("Failed to complete project");

        // Try to contribute to completed project
        let result = manager.contribute_resonance(project_id, &entity2, 5.0);

        assert_eq!(
            result,
            Err(ManifestationError::ProjectAlreadyComplete(project_id))
        );
    }

    #[test]
    fn test_update_completed_project() {
        let mut manager = ManifestationManager::new();

        let entity1 = create_test_entity(1, 0.6, 0.4);
        let entity2 = create_test_entity(2, 0.55, 0.45);

        let entities = vec![&entity1, &entity2];
        let project_id = manager
            .create_project(&entities, StructureType::Temple, [0.0, 0.0, 0.0], 0.0)
            .expect("Failed to create project");

        // Complete project
        let project = manager.get_project(project_id).unwrap();
        let required_resonance = project.structure.required_resonance;

        manager
            .contribute_resonance(project_id, &entity1, required_resonance * 2.0)
            .expect("Failed to complete project");

        // Try to update completed project
        let result = manager.update_project(project_id, &entities);

        assert_eq!(
            result,
            Err(ManifestationError::ProjectAlreadyComplete(project_id))
        );
    }

    #[test]
    fn test_requirements_check() {
        let mut manager = ManifestationManager::new();

        let entity1 = create_test_entity(1, 0.6, 0.4);
        let entity2 = create_test_entity(2, 0.55, 0.45);

        let entities = vec![&entity1, &entity2];
        let project_id = manager
            .create_project(&entities, StructureType::Temple, [0.0, 0.0, 0.0], 0.0)
            .expect("Failed to create project");

        // Initially, requirements may not be met
        let requirements_met = manager.check_requirements(project_id).unwrap();
        assert!(!requirements_met);

        // Contribute some resonance
        manager
            .contribute_resonance(project_id, &entity1, 5.0)
            .expect("Failed to contribute resonance");

        // Requirements should still not be met
        let requirements_met = manager.check_requirements(project_id).unwrap();
        assert!(!requirements_met);
    }

    #[test]
    fn test_requirements_check_nonexistent_project() {
        let manager = ManifestationManager::new();

        let result = manager.check_requirements(999);

        assert_eq!(result, Err(ManifestationError::ProjectNotFound(999)));
    }

    #[test]
    fn test_project_cancellation() {
        let mut manager = ManifestationManager::new();

        let entity1 = create_test_entity(1, 0.6, 0.4);
        let entity2 = create_test_entity(2, 0.55, 0.45);

        let entities = vec![&entity1, &entity2];
        let project_id = manager
            .create_project(&entities, StructureType::Temple, [0.0, 0.0, 0.0], 0.0)
            .expect("Failed to create project");

        assert_eq!(manager.active_project_count(), 1);

        manager
            .cancel_project(project_id)
            .expect("Failed to cancel project");

        assert_eq!(manager.active_project_count(), 0);
        assert!(!manager.has_project(project_id));
    }

    #[test]
    fn test_cancel_nonexistent_project() {
        let mut manager = ManifestationManager::new();

        let result = manager.cancel_project(999);

        assert_eq!(result, Err(ManifestationError::ProjectNotFound(999)));
    }

    #[test]
    fn test_get_active_projects() {
        let mut manager = ManifestationManager::new();

        let entity1 = create_test_entity(1, 0.6, 0.4);
        let entity2 = create_test_entity(2, 0.55, 0.45);

        let entities1 = vec![&entity1];
        let entities2 = vec![&entity2];

        let project_id1 = manager
            .create_project(&entities1, StructureType::Temple, [0.0, 0.0, 0.0], 0.0)
            .expect("Failed to create project 1");

        let project_id2 = manager
            .create_project(&entities2, StructureType::Habitat, [10.0, 20.0, 30.0], 5.0)
            .expect("Failed to create project 2");

        let active_projects = manager.list_active_projects();

        assert_eq!(active_projects.len(), 2);
        assert!(active_projects.contains(&project_id1));
        assert!(active_projects.contains(&project_id2));
    }

    #[test]
    fn test_get_completed_structures() {
        let mut manager = ManifestationManager::new();

        let entity1 = create_test_entity(1, 0.6, 0.4);
        let entity2 = create_test_entity(2, 0.55, 0.45);

        let entities1 = vec![&entity1];
        let entities2 = vec![&entity2];

        let project_id1 = manager
            .create_project(&entities1, StructureType::Temple, [0.0, 0.0, 0.0], 0.0)
            .expect("Failed to create project 1");

        let project_id2 = manager
            .create_project(&entities2, StructureType::Habitat, [10.0, 20.0, 30.0], 5.0)
            .expect("Failed to create project 2");

        // Complete both projects
        let project1 = manager.get_project(project_id1).unwrap();
        let required1 = project1.structure.required_resonance;
        manager
            .contribute_resonance(project_id1, &entity1, required1 * 2.0)
            .expect("Failed to complete project 1");

        let project2 = manager.get_project(project_id2).unwrap();
        let required2 = project2.structure.required_resonance;
        manager
            .contribute_resonance(project_id2, &entity2, required2 * 2.0)
            .expect("Failed to complete project 2");

        let completed_structures = manager.list_completed_structures();

        assert_eq!(completed_structures.len(), 2);
        assert!(completed_structures.contains(&project_id1));
        assert!(completed_structures.contains(&project_id2));
    }

    #[test]
    fn test_get_completed_structure() {
        let mut manager = ManifestationManager::new();

        let entity1 = create_test_entity(1, 0.6, 0.4);

        let entities = vec![&entity1];
        let project_id = manager
            .create_project(&entities, StructureType::Temple, [0.0, 0.0, 0.0], 0.0)
            .expect("Failed to create project");

        // Complete project
        let project = manager.get_project(project_id).unwrap();
        let required_resonance = project.structure.required_resonance;
        manager
            .contribute_resonance(project_id, &entity1, required_resonance * 2.0)
            .expect("Failed to complete project");

        let structure = manager.get_completed_structure(project_id).unwrap();
        assert_eq!(structure.structure_id, project_id);
        assert!(structure.is_complete());
    }

    #[test]
    fn test_get_completed_structure_nonexistent() {
        let manager = ManifestationManager::new();

        let result = manager.get_completed_structure(999);

        assert_eq!(result, Err(ManifestationError::ProjectNotFound(999)));
    }

    #[test]
    fn test_multiple_projects() {
        let mut manager = ManifestationManager::new();

        let entity1 = create_test_entity(1, 0.6, 0.4);
        let entity2 = create_test_entity(2, 0.55, 0.45);
        let entity3 = create_test_entity(3, 0.65, 0.35);

        let entities1 = vec![&entity1];
        let entities2 = vec![&entity2];
        let entities3 = vec![&entity3];

        let project_id1 = manager
            .create_project(&entities1, StructureType::Temple, [0.0, 0.0, 0.0], 0.0)
            .expect("Failed to create project 1");

        let project_id2 = manager
            .create_project(&entities2, StructureType::Habitat, [10.0, 20.0, 30.0], 5.0)
            .expect("Failed to create project 2");

        let project_id3 = manager
            .create_project(&entities3, StructureType::Library, [50.0, 60.0, 70.0], 10.0)
            .expect("Failed to create project 3");

        assert_eq!(manager.active_project_count(), 3);
        assert!(manager.has_project(project_id1));
        assert!(manager.has_project(project_id2));
        assert!(manager.has_project(project_id3));

        // Verify project IDs are sequential
        assert_eq!(project_id1, 1);
        assert_eq!(project_id2, 2);
        assert_eq!(project_id3, 3);
    }

    #[test]
    fn test_project_duration() {
        let mut manager = ManifestationManager::new();

        let entity1 = create_test_entity(1, 0.6, 0.4);

        let entities = vec![&entity1];
        let start_time = 100.0;
        let project_id = manager
            .create_project(
                &entities,
                StructureType::Temple,
                [0.0, 0.0, 0.0],
                start_time,
            )
            .expect("Failed to create project");

        let project = manager.get_project(project_id).unwrap();

        // Duration at start time should be 0
        assert_eq!(project.duration(start_time), 0.0);

        // Duration after 50 time units should be 50
        assert_eq!(project.duration(150.0), 50.0);

        // Duration before start time should be 0
        assert_eq!(project.duration(50.0), 0.0);
    }

    #[test]
    fn test_contributor_count() {
        let mut manager = ManifestationManager::new();

        let entity1 = create_test_entity(1, 0.6, 0.4);
        let entity2 = create_test_entity(2, 0.55, 0.45);
        let entity3 = create_test_entity(3, 0.65, 0.35);

        let entities = vec![&entity1, &entity2, &entity3];
        let project_id = manager
            .create_project(&entities, StructureType::Temple, [0.0, 0.0, 0.0], 0.0)
            .expect("Failed to create project");

        let project = manager.get_project(project_id).unwrap();
        assert_eq!(project.contributor_count(), 3);
    }

    #[test]
    fn test_different_structure_types() {
        let mut manager = ManifestationManager::new();

        let entity1 = create_test_entity(1, 0.6, 0.4);

        let entities = vec![&entity1];

        // Create different structure types
        let temple_id = manager
            .create_project(&entities, StructureType::Temple, [0.0, 0.0, 0.0], 0.0)
            .expect("Failed to create temple");

        let habitat_id = manager
            .create_project(&entities, StructureType::Habitat, [10.0, 10.0, 10.0], 0.0)
            .expect("Failed to create habitat");

        let library_id = manager
            .create_project(&entities, StructureType::Library, [20.0, 20.0, 20.0], 0.0)
            .expect("Failed to create library");

        // Verify all projects exist
        assert!(manager.has_project(temple_id));
        assert!(manager.has_project(habitat_id));
        assert!(manager.has_project(library_id));

        // Verify different structure types
        let temple_project = manager.get_project(temple_id).unwrap();
        let habitat_project = manager.get_project(habitat_id).unwrap();
        let library_project = manager.get_project(library_id).unwrap();

        assert_eq!(
            temple_project.structure.structure_type,
            StructureType::Temple
        );
        assert_eq!(
            habitat_project.structure.structure_type,
            StructureType::Habitat
        );
        assert_eq!(
            library_project.structure.structure_type,
            StructureType::Library
        );

        // Different structure types should have different resonance requirements
        let temple_required = temple_project.structure.required_resonance;
        let habitat_required = habitat_project.structure.required_resonance;
        let library_required = library_project.structure.required_resonance;

        assert_ne!(temple_required, habitat_required);
        assert_ne!(habitat_required, library_required);
    }

    #[test]
    fn test_error_display() {
        let error = ManifestationError::ProjectNotFound(42);
        assert_eq!(format!("{}", error), "Project 42 not found");

        let error = ManifestationError::InsufficientResonance(100.0, 50.0);
        assert_eq!(
            format!("{}", error),
            "Insufficient resonance: required 100, current 50"
        );

        let error = ManifestationError::ProjectAlreadyComplete(99);
        assert_eq!(format!("{}", error), "Project 99 is already complete");
    }

    #[test]
    fn test_build_progress_clamping() {
        let mut manager = ManifestationManager::new();

        let entity1 = create_test_entity(1, 0.6, 0.4);

        let entities = vec![&entity1];
        let project_id = manager
            .create_project(&entities, StructureType::Temple, [0.0, 0.0, 0.0], 0.0)
            .expect("Failed to create project");

        // Get the project to determine required resonance
        let project = manager.get_project(project_id).unwrap();
        let required_resonance = project.structure.required_resonance;

        // Contribute more than required resonance
        manager
            .contribute_resonance(project_id, &entity1, required_resonance * 3.0)
            .expect("Failed to contribute resonance");

        // Verify project moved to completed structures
        assert_eq!(manager.active_project_count(), 0);
        assert_eq!(manager.completed_structure_count(), 1);

        // Verify completed structure has exactly 1.0 progress
        let structure = manager.get_completed_structure(project_id).unwrap();
        assert_eq!(structure.build_progress, 1.0);
    }

    #[test]
    fn test_structure_position_tracking() {
        let mut manager = ManifestationManager::new();

        let entity1 = create_test_entity(1, 0.6, 0.4);

        let entities = vec![&entity1];
        let position = [100.0, 200.0, 300.0];
        let project_id = manager
            .create_project(&entities, StructureType::Temple, position, 0.0)
            .expect("Failed to create project");

        let project = manager.get_project(project_id).unwrap();
        assert_eq!(project.structure.position, position);
    }

    #[test]
    fn test_stability_update() {
        let mut manager = ManifestationManager::new();

        let entity1 = create_test_entity(1, 0.6, 0.4);

        let entities = vec![&entity1];
        let project_id = manager
            .create_project(&entities, StructureType::Temple, [0.0, 0.0, 0.0], 0.0)
            .expect("Failed to create project");

        let project = manager.get_project(project_id).unwrap();
        let initial_stability = project.structure.stability;

        // Contribute resonance (should update stability)
        manager
            .contribute_resonance(project_id, &entity1, 5.0)
            .expect("Failed to contribute resonance");

        // Stability should be updated based on coherence
        let project = manager.get_project(project_id).unwrap();
        assert!(project.structure.stability >= 0.0);
        assert!(project.structure.stability <= 1.0);
    }

    #[test]
    fn test_empty_active_projects_list() {
        let manager = ManifestationManager::new();

        let active_projects = manager.list_active_projects();
        assert_eq!(active_projects.len(), 0);
    }

    #[test]
    fn test_empty_completed_structures_list() {
        let manager = ManifestationManager::new();

        let completed_structures = manager.list_completed_structures();
        assert_eq!(completed_structures.len(), 0);
    }

    #[test]
    fn test_project_structure_relationship() {
        let mut manager = ManifestationManager::new();

        let entity1 = create_test_entity(1, 0.6, 0.4);
        let entity2 = create_test_entity(2, 0.55, 0.45);

        let entities = vec![&entity1, &entity2];
        let project_id = manager
            .create_project(&entities, StructureType::Temple, [0.0, 0.0, 0.0], 0.0)
            .expect("Failed to create project");

        let project = manager.get_project(project_id).unwrap();

        // Verify structure ID matches project ID
        assert_eq!(project.structure.structure_id, project_id);

        // Verify structure has contributors
        assert!(project.structure.contributor_count() > 0);

        // Verify project has contributing entities
        assert!(project.contributor_count() > 0);
    }
}
