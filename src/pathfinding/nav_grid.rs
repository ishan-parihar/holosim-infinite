//! Navigation Grid for Terrain-Based Pathfinding
//!
//! This module generates a 2D navigation grid from the lithosphere's crust data,
//! providing cost-based navigation that respects terrain types and obstacles.
//!
//! The nav grid maps lat/lon coordinates to grid cells with associated movement
//! costs, enabling A* pathfinding across procedural terrain.

use crate::holographic::field_address::Vector3;
use crate::planet::lithosphere::{Lithosphere, TerrainClass};

/// Default grid resolution (degrees per cell)
const DEFAULT_RESOLUTION: f64 = 5.0;

/// Elevation threshold for mountains (very high movement cost)
const MOUNTAIN_THRESHOLD: f64 = 3000.0;

/// Movement cost multiplier for deep water (impassable)
const DEEP_WATER_COST: f64 = f64::INFINITY;

/// Movement cost multiplier for shallow water
const SHALLOW_WATER_COST: f64 = 10.0;

/// Movement cost multiplier for beach
const BEACH_COST: f64 = 1.5;

/// Movement cost multiplier for lowland/plains
const LOWLAND_COST: f64 = 1.0;

/// Movement cost multiplier for hills
const HILL_COST: f64 = 2.0;

/// Movement cost multiplier for mountains
const MOUNTAIN_COST: f64 = 5.0;

/// Movement cost multiplier for highlands
const HIGHLAND_COST: f64 = 3.0;

/// Movement cost multiplier for plateaus
const PLATEAU_COST: f64 = 2.5;

/// Movement cost multiplier for desert
const DESERT_COST: f64 = 1.8;

/// Movement cost multiplier for tundra
const TUNDRA_COST: f64 = 2.2;

/// Movement cost multiplier for ice
const ICE_COST: f64 = 3.0;

/// Latitude range for the grid
const LAT_MIN: i32 = -90;
const LAT_MAX: i32 = 90;
const LON_MIN: i32 = -180;
const LON_MAX: i32 = 180;

/// A single cell in the navigation grid
#[derive(Debug, Clone, Copy)]
pub struct NavCell {
    /// Grid coordinates
    pub grid_x: usize,
    pub grid_y: usize,
    /// World position (lat, lon)
    pub lat: f64,
    pub lon: f64,
    /// Elevation at this position (meters)
    pub elevation: f64,
    /// Terrain classification
    pub terrain: TerrainClass,
    /// Movement cost multiplier (1.0 = normal, higher = harder, inf = impassable)
    pub movement_cost: f64,
    /// Whether this cell is traversable
    pub is_passable: bool,
}

/// Navigation grid generated from terrain data
#[derive(Debug, Clone)]
pub struct NavGrid {
    /// Grid cells stored in row-major order (row = lat, col = lon)
    pub cells: Vec<NavCell>,
    /// Number of columns (longitude divisions)
    pub cols: usize,
    /// Number of rows (latitude divisions)
    pub rows: usize,
    /// Resolution in degrees per cell
    pub resolution: f64,
}

impl NavGrid {
    /// Create a new navigation grid from a lithosphere
    pub fn from_lithosphere(litho: &Lithosphere, resolution: Option<f64>) -> Self {
        let res = resolution.unwrap_or(DEFAULT_RESOLUTION);

        let rows = ((LAT_MAX - LAT_MIN) as f64 / res) as usize;
        let cols = ((LON_MAX - LON_MIN) as f64 / res) as usize;

        let mut cells = Vec::with_capacity(rows * cols);

        for row in 0..rows {
            for col in 0..cols {
                let lat = LAT_MIN as f64 + row as f64 * res;
                let lon = LON_MIN as f64 + col as f64 * res;

                let elevation = litho.elevation_at(lat, lon);
                let terrain = litho.terrain_at(lat, lon);
                let (movement_cost, is_passable) = Self::compute_terrain_cost(&terrain, elevation);

                cells.push(NavCell {
                    grid_x: col,
                    grid_y: row,
                    lat,
                    lon,
                    elevation,
                    terrain,
                    movement_cost,
                    is_passable,
                });
            }
        }

        NavGrid {
            cells,
            cols,
            rows,
            resolution: res,
        }
    }

    /// Create a simple flat navigation grid for testing
    pub fn new_flat(rows: usize, cols: usize, resolution: f64) -> Self {
        let mut cells = Vec::with_capacity(rows * cols);

        for row in 0..rows {
            for col in 0..cols {
                let lat = LAT_MIN as f64 + row as f64 * resolution;
                let lon = LON_MIN as f64 + col as f64 * resolution;

                cells.push(NavCell {
                    grid_x: col,
                    grid_y: row,
                    lat,
                    lon,
                    elevation: 100.0,
                    terrain: TerrainClass::Lowland,
                    movement_cost: LOWLAND_COST,
                    is_passable: true,
                });
            }
        }

        NavGrid {
            cells,
            cols,
            rows,
            resolution,
        }
    }

    /// Compute terrain cost and passability from terrain class and elevation
    fn compute_terrain_cost(terrain: &TerrainClass, elevation: f64) -> (f64, bool) {
        let cost = match terrain {
            TerrainClass::DeepOcean => DEEP_WATER_COST,
            TerrainClass::ShallowOcean => SHALLOW_WATER_COST,
            TerrainClass::Beach => BEACH_COST,
            TerrainClass::Lowland => LOWLAND_COST,
            TerrainClass::Hill => HILL_COST,
            TerrainClass::Mountain => {
                if elevation > MOUNTAIN_THRESHOLD {
                    DEEP_WATER_COST // Very high mountains are impassable
                } else {
                    MOUNTAIN_COST
                }
            }
            TerrainClass::Highland => HIGHLAND_COST,
            TerrainClass::Plateau => PLATEAU_COST,
            TerrainClass::Desert => DESERT_COST,
            TerrainClass::Tundra => TUNDRA_COST,
            TerrainClass::Ice => ICE_COST,
        };

        let is_passable = cost.is_finite();
        (cost, is_passable)
    }

    /// Get cell at grid coordinates
    pub fn cell_at(&self, row: usize, col: usize) -> Option<&NavCell> {
        if row < self.rows && col < self.cols {
            Some(&self.cells[row * self.cols + col])
        } else {
            None
        }
    }

    /// Get cell at world coordinates (lat, lon), rounded to nearest grid cell
    pub fn cell_at_world(&self, lat: f64, lon: f64) -> Option<&NavCell> {
        let row = ((lat - LAT_MIN as f64) / self.resolution).round() as usize;
        let col = ((lon - LON_MIN as f64) / self.resolution).round() as usize;
        self.cell_at(row, col)
    }

    /// Get cell index from grid coordinates
    pub fn index(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    /// Convert world coordinates to grid coordinates
    pub fn world_to_grid(&self, lat: f64, lon: f64) -> (usize, usize) {
        let row = ((lat - LAT_MIN as f64) / self.resolution).round() as usize;
        let col = ((lon - LON_MIN as f64) / self.resolution).round() as usize;
        (
            row.min(self.rows.saturating_sub(1)),
            col.min(self.cols.saturating_sub(1)),
        )
    }

    /// Convert grid coordinates to world coordinates (center of cell)
    pub fn grid_to_world(&self, row: usize, col: usize) -> (f64, f64) {
        let lat = LAT_MIN as f64 + row as f64 * self.resolution + self.resolution / 2.0;
        let lon = LON_MIN as f64 + col as f64 * self.resolution + self.resolution / 2.0;
        (lat, lon)
    }

    /// Convert grid coordinates to 3D position
    pub fn grid_to_vec3(&self, row: usize, col: usize, elevation: Option<f64>) -> Vector3 {
        let (lat, lon) = self.grid_to_world(row, col);
        let elev =
            elevation.unwrap_or_else(|| self.cell_at(row, col).map(|c| c.elevation).unwrap_or(0.0));
        // Convert lat/lon/elevation to a local coordinate system
        // Using a simple equirectangular projection for local navigation
        let x = lon;
        let y = elev / 1000.0; // Scale elevation to reasonable range
        let z = lat;
        Vector3::new(x, y, z)
    }

    /// Check if a grid cell is passable
    pub fn is_passable(&self, row: usize, col: usize) -> bool {
        self.cell_at(row, col)
            .map(|c| c.is_passable)
            .unwrap_or(false)
    }

    /// Get the dimensions of the grid as (rows, cols)
    pub fn dimensions(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }
}

impl Default for NavGrid {
    fn default() -> Self {
        Self::new_flat(36, 72, DEFAULT_RESOLUTION)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flat_grid_creation() {
        let grid = NavGrid::new_flat(10, 20, 5.0);
        assert_eq!(grid.rows, 10);
        assert_eq!(grid.cols, 20);
        assert_eq!(grid.cells.len(), 200);
    }

    #[test]
    fn test_grid_cell_access() {
        let grid = NavGrid::new_flat(10, 20, 5.0);
        let cell = grid.cell_at(5, 10);
        assert!(cell.is_some());
        let cell = cell.unwrap();
        assert_eq!(cell.grid_x, 10);
        assert_eq!(cell.grid_y, 5);
    }

    #[test]
    fn test_grid_bounds_checking() {
        let grid = NavGrid::new_flat(10, 20, 5.0);
        assert!(grid.cell_at(10, 0).is_none()); // Out of bounds
        assert!(grid.cell_at(0, 20).is_none()); // Out of bounds
        assert!(grid.cell_at(9, 19).is_some()); // Last valid cell
    }

    #[test]
    fn test_world_to_grid_conversion() {
        let grid = NavGrid::new_flat(36, 72, 5.0);
        let (row, col) = grid.world_to_grid(-90.0, -180.0);
        assert_eq!(row, 0);
        assert_eq!(col, 0);

        let (row, col) = grid.world_to_grid(85.0, 175.0);
        assert_eq!(row, 35);
        assert_eq!(col, 71);
    }

    #[test]
    fn test_grid_to_world_conversion() {
        let grid = NavGrid::new_flat(36, 72, 5.0);
        let (lat, lon) = grid.grid_to_world(0, 0);
        assert!((lat - (-87.5)).abs() < 0.001); // Center of first cell
        assert!((lon - (-177.5)).abs() < 0.001);
    }

    #[test]
    fn test_all_cells_passable_in_flat_grid() {
        let grid = NavGrid::new_flat(10, 10, 5.0);
        for row in 0..grid.rows {
            for col in 0..grid.cols {
                assert!(
                    grid.is_passable(row, col),
                    "Cell ({}, {}) should be passable",
                    row,
                    col
                );
            }
        }
    }

    #[test]
    fn test_nav_grid_from_lithosphere() {
        let litho = Lithosphere::new();
        let grid = NavGrid::from_lithosphere(&litho, None);
        assert!(!grid.cells.is_empty());
        assert_eq!(grid.rows, 36);
        assert_eq!(grid.cols, 72);
    }

    #[test]
    fn test_grid_to_vec3() {
        let grid = NavGrid::new_flat(10, 10, 5.0);
        let pos = grid.grid_to_vec3(0, 0, Some(100.0));
        assert!((pos.x - (-177.5)).abs() < 0.001);
        assert!((pos.z - (-87.5)).abs() < 0.001);
        assert!((pos.y - 0.1).abs() < 0.001); // 100m / 1000
    }
}
