//! Spatial Partitioning for Entity Interactions
//!
//! This module implements a 3D grid-based spatial partitioning system to optimize
//! entity interaction queries. Instead of O(n²) complexity for all entity interactions,
//! spatial partitioning reduces complexity to O(n) or O(n log n) by only checking
//! entities in nearby grid cells.
//!
//! # Performance Benefits
//! - Reduces interaction complexity from O(n²) to O(n) for uniform distributions
//! - Optimizes holographic field updates (dominant bottleneck at ~55% of time)
//! - Enables efficient range queries for nearby entities
//! - Supports dynamic entity movement with efficient cell updates

use crate::energy_fields::Vector3;
use crate::natural_laws::Float;
use std::collections::{HashMap, HashSet};

/// 3D cell coordinates in the spatial grid
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CellCoord {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl CellCoord {
    /// Create a new cell coordinate
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        CellCoord { x, y, z }
    }

    /// Convert a position to cell coordinates
    pub fn from_position(position: &Vector3, cell_size: Float) -> Self {
        CellCoord {
            x: (position.x / cell_size).floor() as i32,
            y: (position.y / cell_size).floor() as i32,
            z: (position.z / cell_size).floor() as i32,
        }
    }

    /// Get neighboring cells (including diagonals)
    pub fn neighbors(&self) -> Vec<CellCoord> {
        let mut neighbors = Vec::new();
        for dx in -1i32..=1 {
            for dy in -1i32..=1 {
                for dz in -1i32..=1 {
                    if dx == 0 && dy == 0 && dz == 0 {
                        continue; // Skip self
                    }
                    neighbors.push(CellCoord::new(self.x + dx, self.y + dy, self.z + dz));
                }
            }
        }
        neighbors
    }
}

/// Entity with spatial position
#[derive(Debug, Clone)]
pub struct SpatialEntity {
    /// Entity ID
    pub id: u64,

    /// Position in 3D space
    pub position: Vector3,

    /// Interaction radius (how far this entity can influence others)
    pub interaction_radius: Float,

    /// Current cell coordinate
    pub cell: Option<CellCoord>,
}

impl SpatialEntity {
    /// Create a new spatial entity
    pub fn new(id: u64, position: Vector3, interaction_radius: Float) -> Self {
        SpatialEntity {
            id,
            position,
            interaction_radius,
            cell: None,
        }
    }

    /// Update entity position
    pub fn update_position(&mut self, new_position: Vector3) {
        self.position = new_position;
    }

    /// Calculate distance to another entity
    pub fn distance_to(&self, other: &SpatialEntity) -> Float {
        let dx = self.position.x - other.position.x;
        let dy = self.position.y - other.position.y;
        let dz = self.position.z - other.position.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Check if this entity is within interaction range of another
    pub fn is_in_range(&self, other: &SpatialEntity) -> bool {
        let distance = self.distance_to(other);
        distance <= (self.interaction_radius + other.interaction_radius)
    }
}

/// Statistics for spatial partitioning
#[derive(Debug, Clone)]
pub struct SpatialPartitionStats {
    /// Total number of entities
    pub total_entities: usize,

    /// Number of occupied cells
    pub occupied_cells: usize,

    /// Average entities per cell
    pub avg_entities_per_cell: Float,

    /// Maximum entities in any single cell
    pub max_entities_in_cell: usize,

    /// Cell size
    pub cell_size: Float,

    /// Total grid extent
    pub grid_extent: (i32, i32, i32, i32, i32, i32), // (min_x, max_x, min_y, max_y, min_z, max_z)
}

/// 3D Spatial Partitioning for Entity Interactions
///
/// Divides space into a 3D grid of cells. Each cell contains the entities
/// that are currently in that region of space. This allows for efficient
/// range queries and interaction calculations.
#[derive(Debug, Clone)]
pub struct SpatialPartition {
    /// Grid cells mapping cell coordinates to entity IDs
    cells: HashMap<CellCoord, Vec<u64>>,

    /// Entity lookup by ID
    entities: HashMap<u64, SpatialEntity>,

    /// Size of each cell in world units
    cell_size: Float,

    /// Statistics tracking
    stats: SpatialPartitionStats,
}

impl SpatialPartition {
    /// Create a new spatial partition with specified cell size
    ///
    /// # Arguments
    /// * `cell_size` - Size of each grid cell in world units
    ///
    /// # Returns
    /// New spatial partition instance
    ///
    /// # Example
    /// ```
    /// let partition = SpatialPartition::new(10.0); // 10x10x10 cells
    /// ```
    pub fn new(cell_size: Float) -> Self {
        assert!(cell_size > 0.0, "Cell size must be positive");

        SpatialPartition {
            cells: HashMap::new(),
            entities: HashMap::new(),
            cell_size,
            stats: SpatialPartitionStats {
                total_entities: 0,
                occupied_cells: 0,
                avg_entities_per_cell: 0.0,
                max_entities_in_cell: 0,
                cell_size,
                grid_extent: (0, 0, 0, 0, 0, 0),
            },
        }
    }

    /// Insert an entity into the spatial partition
    ///
    /// # Arguments
    /// * `entity` - The entity to insert
    ///
    /// # Returns
    /// `true` if the entity was inserted, `false` if an entity with the same ID already exists
    pub fn insert(&mut self, mut entity: SpatialEntity) -> bool {
        // Check if entity already exists
        if self.entities.contains_key(&entity.id) {
            return false;
        }

        // Calculate cell coordinate
        let cell = CellCoord::from_position(&entity.position, self.cell_size);

        // Set entity's cell
        entity.cell = Some(cell);

        // Add entity to cell
        self.cells
            .entry(cell)
            .or_insert_with(Vec::new)
            .push(entity.id);

        // Store entity
        self.entities.insert(entity.id, entity);

        self.update_stats();
        true
    }

    /// Remove an entity from the spatial partition
    ///
    /// # Arguments
    /// * `entity_id` - ID of the entity to remove
    ///
    /// # Returns
    /// `Some(entity)` if the entity was found and removed, `None` otherwise
    pub fn remove(&mut self, entity_id: u64) -> Option<SpatialEntity> {
        let entity = self.entities.remove(&entity_id)?;

        // Remove from cell
        if let Some(cell_entities) = self.cells.get_mut(&entity.cell?) {
            cell_entities.retain(|&id| id != entity_id);

            // Remove empty cell
            if cell_entities.is_empty() {
                self.cells.remove(&entity.cell?);
            }
        }

        self.update_stats();
        Some(entity)
    }

    /// Update an entity's position
    ///
    /// # Arguments
    /// * `entity_id` - ID of the entity to update
    /// * `new_position` - New position for the entity
    ///
    /// # Returns
    /// `true` if the entity was updated, `false` if the entity was not found
    pub fn update_position(&mut self, entity_id: u64, new_position: Vector3) -> bool {
        let entity = match self.entities.get_mut(&entity_id) {
            Some(e) => e,
            None => return false,
        };

        // Calculate old and new cells
        let old_cell = entity.cell;
        let new_cell = CellCoord::from_position(&new_position, self.cell_size);

        // Update position
        entity.position = new_position;

        // If cell changed, move entity to new cell
        if old_cell != Some(new_cell) {
            // Remove from old cell
            if let Some(old_cell_coord) = old_cell {
                if let Some(cell_entities) = self.cells.get_mut(&old_cell_coord) {
                    cell_entities.retain(|&id| id != entity_id);

                    // Remove empty cell
                    if cell_entities.is_empty() {
                        self.cells.remove(&old_cell_coord);
                    }
                }
            }

            // Add to new cell
            self.cells
                .entry(new_cell)
                .or_insert_with(Vec::new)
                .push(entity_id);

            entity.cell = Some(new_cell);
        }

        self.update_stats();
        true
    }

    /// Query all entities within a certain distance of a position
    ///
    /// # Arguments
    /// * `position` - Center position for the query
    /// * `radius` - Query radius
    ///
    /// # Returns
    /// Vector of entity IDs within the specified radius
    pub fn query_range(&self, position: Vector3, radius: Float) -> Vec<u64> {
        let mut result = Vec::new();
        let center_cell = CellCoord::from_position(&position, self.cell_size);

        // Calculate how many cells to check
        let cells_to_check = (radius / self.cell_size).ceil() as i32 + 1;

        // Check all cells that might contain entities within range
        for dx in -cells_to_check..=cells_to_check {
            for dy in -cells_to_check..=cells_to_check {
                for dz in -cells_to_check..=cells_to_check {
                    let cell =
                        CellCoord::new(center_cell.x + dx, center_cell.y + dy, center_cell.z + dz);

                    if let Some(entity_ids) = self.cells.get(&cell) {
                        for &entity_id in entity_ids {
                            if let Some(entity) = self.entities.get(&entity_id) {
                                let dx = entity.position.x - position.x;
                                let dy = entity.position.y - position.y;
                                let dz = entity.position.z - position.z;
                                let distance_sq = dx * dx + dy * dy + dz * dz;

                                if distance_sq <= radius * radius {
                                    result.push(entity_id);
                                }
                            }
                        }
                    }
                }
            }
        }

        result
    }

    /// Find all entities that are within interaction range of a given entity
    ///
    /// # Arguments
    /// * `entity_id` - ID of the entity to find neighbors for
    ///
    /// # Returns
    /// Vector of entity IDs that are within interaction range
    pub fn find_neighbors(&self, entity_id: u64) -> Vec<u64> {
        let entity = match self.entities.get(&entity_id) {
            Some(e) => e,
            None => return Vec::new(),
        };

        // Use query_range with the entity's interaction radius
        let mut neighbors = self.query_range(entity.position, entity.interaction_radius);

        // Remove self from results
        neighbors.retain(|&id| id != entity_id);

        neighbors
    }

    /// Get all entities in a specific cell
    ///
    /// # Arguments
    /// * `cell` - Cell coordinate
    ///
    /// # Returns
    /// Vector of entity IDs in the cell, or empty vector if cell is empty
    pub fn get_entities_in_cell(&self, cell: CellCoord) -> Vec<u64> {
        self.cells.get(&cell).cloned().unwrap_or_default()
    }

    /// Get a reference to an entity
    ///
    /// # Arguments
    /// * `entity_id` - ID of the entity
    ///
    /// # Returns
    /// `Some(&entity)` if the entity exists, `None` otherwise
    pub fn get_entity(&self, entity_id: u64) -> Option<&SpatialEntity> {
        self.entities.get(&entity_id)
    }

    /// Get all entity IDs
    ///
    /// # Returns
    /// Vector of all entity IDs in the partition
    pub fn all_entities(&self) -> Vec<u64> {
        self.entities.keys().cloned().collect()
    }

    /// Get all occupied cells
    ///
    /// # Returns
    /// Vector of all cell coordinates that contain at least one entity
    pub fn occupied_cells(&self) -> Vec<CellCoord> {
        self.cells.keys().cloned().collect()
    }

    /// Clear all entities from the partition
    pub fn clear(&mut self) {
        self.cells.clear();
        self.entities.clear();
        self.update_stats();
    }

    /// Get the number of entities in the partition
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    /// Get the number of occupied cells
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    /// Get the cell size
    pub fn cell_size(&self) -> Float {
        self.cell_size
    }

    /// Get statistics about the spatial partition
    pub fn get_stats(&self) -> &SpatialPartitionStats {
        &self.stats
    }

    /// Update statistics
    fn update_stats(&mut self) {
        let total_entities = self.entities.len();
        let occupied_cells = self.cells.len();

        let avg_entities_per_cell = if occupied_cells > 0 {
            total_entities as Float / occupied_cells as Float
        } else {
            0.0
        };

        let max_entities_in_cell = self.cells.values().map(|v| v.len()).max().unwrap_or(0);

        // Calculate grid extent
        let (min_x, max_x, min_y, max_y, min_z, max_z) = if occupied_cells > 0 {
            let mut min_x = i32::MAX;
            let mut max_x = i32::MIN;
            let mut min_y = i32::MAX;
            let mut max_y = i32::MIN;
            let mut min_z = i32::MAX;
            let mut max_z = i32::MIN;

            for cell in self.cells.keys() {
                min_x = min_x.min(cell.x);
                max_x = max_x.max(cell.x);
                min_y = min_y.min(cell.y);
                max_y = max_y.max(cell.y);
                min_z = min_z.min(cell.z);
                max_z = max_z.max(cell.z);
            }

            (min_x, max_x, min_y, max_y, min_z, max_z)
        } else {
            (0, 0, 0, 0, 0, 0)
        };

        self.stats = SpatialPartitionStats {
            total_entities,
            occupied_cells,
            avg_entities_per_cell,
            max_entities_in_cell,
            cell_size: self.cell_size,
            grid_extent: (min_x, max_x, min_y, max_y, min_z, max_z),
        };
    }

    /// Calculate potential interactions (pairs of entities that might interact)
    ///
    /// This method returns all entity pairs that are within interaction range of each other.
    /// It uses spatial partitioning to efficiently find these pairs without checking all O(n²) combinations.
    ///
    /// # Returns
    /// Vector of (entity_id1, entity_id2) pairs where entities are in range
    pub fn find_potential_interactions(&self) -> Vec<(u64, u64)> {
        let mut interactions = Vec::new();
        let mut processed_pairs = HashSet::new();

        // For each occupied cell, check entities in that cell and neighboring cells
        for (cell, entity_ids) in &self.cells {
            // Check entities within the same cell
            for (i, &id1) in entity_ids.iter().enumerate() {
                for &id2 in entity_ids.iter().skip(i + 1) {
                    // Create ordered pair to avoid duplicates
                    let pair = if id1 < id2 { (id1, id2) } else { (id2, id1) };

                    if !processed_pairs.contains(&pair) {
                        // Check if entities are actually in range
                        if let (Some(e1), Some(e2)) =
                            (self.entities.get(&pair.0), self.entities.get(&pair.1))
                        {
                            if e1.is_in_range(e2) {
                                interactions.push(pair);
                                processed_pairs.insert(pair);
                            }
                        }
                    }
                }
            }

            // Check entities in neighboring cells
            for neighbor_cell in cell.neighbors() {
                if let Some(neighbor_entity_ids) = self.cells.get(&neighbor_cell) {
                    for &id1 in entity_ids {
                        for &id2 in neighbor_entity_ids {
                            // Create ordered pair to avoid duplicates
                            let pair = if id1 < id2 { (id1, id2) } else { (id2, id1) };

                            if !processed_pairs.contains(&pair) {
                                // Check if entities are actually in range
                                if let (Some(e1), Some(e2)) =
                                    (self.entities.get(&pair.0), self.entities.get(&pair.1))
                                {
                                    if e1.is_in_range(e2) {
                                        interactions.push(pair);
                                        processed_pairs.insert(pair);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        interactions
    }
}

impl Default for SpatialPartition {
    fn default() -> Self {
        Self::new(10.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_coord_creation() {
        let coord = CellCoord::new(1, 2, 3);
        assert_eq!(coord.x, 1);
        assert_eq!(coord.y, 2);
        assert_eq!(coord.z, 3);
    }

    #[test]
    fn test_cell_coord_from_position() {
        let position = Vector3::new(15.0, 25.0, 35.0);
        let coord = CellCoord::from_position(&position, 10.0);
        assert_eq!(coord.x, 1);
        assert_eq!(coord.y, 2);
        assert_eq!(coord.z, 3);
    }

    #[test]
    fn test_cell_coord_neighbors() {
        let coord = CellCoord::new(0, 0, 0);
        let neighbors = coord.neighbors();
        assert_eq!(neighbors.len(), 26); // 3^3 - 1
    }

    #[test]
    fn test_spatial_entity_creation() {
        let entity = SpatialEntity::new(1, Vector3::new(0.0, 0.0, 0.0), 10.0);
        assert_eq!(entity.id, 1);
        assert_eq!(entity.position, Vector3::new(0.0, 0.0, 0.0));
        assert_eq!(entity.interaction_radius, 10.0);
    }

    #[test]
    fn test_spatial_entity_distance() {
        let entity1 = SpatialEntity::new(1, Vector3::new(0.0, 0.0, 0.0), 10.0);
        let entity2 = SpatialEntity::new(2, Vector3::new(3.0, 4.0, 0.0), 10.0);
        let distance = entity1.distance_to(&entity2);
        assert!((distance - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_spatial_entity_in_range() {
        let entity1 = SpatialEntity::new(1, Vector3::new(0.0, 0.0, 0.0), 5.0);
        let entity2 = SpatialEntity::new(2, Vector3::new(3.0, 4.0, 0.0), 5.0);
        assert!(entity1.is_in_range(&entity2));

        let entity3 = SpatialEntity::new(3, Vector3::new(11.0, 0.0, 0.0), 5.0);
        assert!(!entity1.is_in_range(&entity3)); // Distance = 11, combined radius = 10
    }

    #[test]
    fn test_spatial_partition_creation() {
        let partition = SpatialPartition::new(10.0);
        assert_eq!(partition.cell_size(), 10.0);
        assert_eq!(partition.entity_count(), 0);
        assert_eq!(partition.cell_count(), 0);
    }

    #[test]
    fn test_spatial_partition_insert() {
        let mut partition = SpatialPartition::new(10.0);
        let entity = SpatialEntity::new(1, Vector3::new(5.0, 5.0, 5.0), 10.0);

        assert!(partition.insert(entity));
        assert_eq!(partition.entity_count(), 1);
        assert_eq!(partition.cell_count(), 1);

        // Test duplicate insertion (create a new entity with same ID)
        let entity2 = SpatialEntity::new(1, Vector3::new(5.0, 5.0, 5.0), 10.0);
        assert!(!partition.insert(entity2));
    }

    #[test]
    fn test_spatial_partition_remove() {
        let mut partition = SpatialPartition::new(10.0);
        let entity = SpatialEntity::new(1, Vector3::new(5.0, 5.0, 5.0), 10.0);

        partition.insert(entity);
        assert_eq!(partition.entity_count(), 1);

        let removed = partition.remove(1);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().id, 1);
        assert_eq!(partition.entity_count(), 0);
        assert_eq!(partition.cell_count(), 0);

        // Test removing non-existent entity
        assert!(partition.remove(999).is_none());
    }

    #[test]
    fn test_spatial_partition_update_position() {
        let mut partition = SpatialPartition::new(10.0);
        let entity = SpatialEntity::new(1, Vector3::new(5.0, 5.0, 5.0), 10.0);

        partition.insert(entity);
        assert!(partition.update_position(1, Vector3::new(15.0, 15.0, 15.0)));

        let updated_entity = partition.get_entity(1).unwrap();
        assert_eq!(updated_entity.position, Vector3::new(15.0, 15.0, 15.0));

        // Test updating non-existent entity
        assert!(!partition.update_position(999, Vector3::new(0.0, 0.0, 0.0)));
    }

    #[test]
    fn test_spatial_partition_query_range() {
        let mut partition = SpatialPartition::new(10.0);

        // Insert entities at different positions
        partition.insert(SpatialEntity::new(1, Vector3::new(0.0, 0.0, 0.0), 10.0));
        partition.insert(SpatialEntity::new(2, Vector3::new(5.0, 0.0, 0.0), 10.0));
        partition.insert(SpatialEntity::new(3, Vector3::new(15.0, 0.0, 0.0), 10.0));

        // Query for entities within 10 units of origin
        let results = partition.query_range(Vector3::new(0.0, 0.0, 0.0), 10.0);

        // Should find entities 1 and 2
        assert_eq!(results.len(), 2);
        assert!(results.contains(&1));
        assert!(results.contains(&2));
        assert!(!results.contains(&3));
    }

    #[test]
    fn test_spatial_partition_find_neighbors() {
        let mut partition = SpatialPartition::new(10.0);

        partition.insert(SpatialEntity::new(1, Vector3::new(0.0, 0.0, 0.0), 10.0));
        partition.insert(SpatialEntity::new(2, Vector3::new(5.0, 0.0, 0.0), 10.0));
        partition.insert(SpatialEntity::new(3, Vector3::new(15.0, 0.0, 0.0), 10.0));

        let neighbors = partition.find_neighbors(1);
        assert_eq!(neighbors.len(), 1);
        assert!(neighbors.contains(&2));
        assert!(!neighbors.contains(&1)); // No self
        assert!(!neighbors.contains(&3)); // Too far
    }

    #[test]
    fn test_spatial_partition_multiple_entities_same_cell() {
        let mut partition = SpatialPartition::new(10.0);

        // Insert multiple entities in the same cell
        for i in 0..10 {
            partition.insert(SpatialEntity::new(i, Vector3::new(5.0, 5.0, 5.0), 10.0));
        }

        assert_eq!(partition.entity_count(), 10);
        assert_eq!(partition.cell_count(), 1);

        let cell = CellCoord::new(0, 0, 0);
        let entities_in_cell = partition.get_entities_in_cell(cell);
        assert_eq!(entities_in_cell.len(), 10);
    }

    #[test]
    fn test_spatial_partition_find_potential_interactions() {
        let mut partition = SpatialPartition::new(10.0);

        // Create entities in a line
        for i in 0..5 {
            partition.insert(SpatialEntity::new(
                i,
                Vector3::new(i as Float * 5.0, 0.0, 0.0),
                6.0, // Interaction radius 6.0
            ));
        }

        let interactions = partition.find_potential_interactions();

        // Each entity should interact with its immediate neighbors
        // 0-1, 1-2, 2-3, 3-4 (4 interactions)
        assert!(interactions.len() >= 4);

        // Verify that interactions are within range
        for (id1, id2) in &interactions {
            let e1 = partition.get_entity(*id1).unwrap();
            let e2 = partition.get_entity(*id2).unwrap();
            assert!(e1.is_in_range(e2));
        }
    }

    #[test]
    fn test_spatial_partition_statistics() {
        let mut partition = SpatialPartition::new(10.0);

        // Insert entities in different cells
        for i in 0..10 {
            partition.insert(SpatialEntity::new(
                i,
                Vector3::new((i * 15) as Float, 0.0, 0.0),
                10.0,
            ));
        }

        let stats = partition.get_stats();
        assert_eq!(stats.total_entities, 10);
        assert_eq!(stats.occupied_cells, 10);
        assert!((stats.avg_entities_per_cell - 1.0).abs() < 1e-6);
        assert_eq!(stats.max_entities_in_cell, 1);
    }

    #[test]
    fn test_spatial_partition_clear() {
        let mut partition = SpatialPartition::new(10.0);

        for i in 0..10 {
            partition.insert(SpatialEntity::new(i, Vector3::new(0.0, 0.0, 0.0), 10.0));
        }

        assert_eq!(partition.entity_count(), 10);

        partition.clear();
        assert_eq!(partition.entity_count(), 0);
        assert_eq!(partition.cell_count(), 0);
    }

    #[test]
    fn test_spatial_partition_cell_size_validation() {
        // Should panic with zero or negative cell size
        let result = std::panic::catch_unwind(|| {
            SpatialPartition::new(0.0);
        });
        assert!(result.is_err());

        let result = std::panic::catch_unwind(|| {
            SpatialPartition::new(-10.0);
        });
        assert!(result.is_err());
    }
}
