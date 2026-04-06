//! Pathfinding module - A* pathfinding on terrain grid

pub mod nav_grid;

use crate::holographic::field_address::Vector3;
use crate::planet::lithosphere::Lithosphere;
pub use nav_grid::{NavCell, NavGrid};
use pathfinding::prelude::astar;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridPos(pub usize, pub usize);

impl Hash for GridPos {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cost(pub u64);

impl Cost {
    pub fn from_f64(v: f64) -> Self {
        Cost((v * 1000.0).round() as u64)
    }

    pub fn to_f64(&self) -> f64 {
        self.0 as f64 / 1000.0
    }
}

impl std::ops::Add for Cost {
    type Output = Cost;
    fn add(self, rhs: Cost) -> Cost {
        Cost(self.0 + rhs.0)
    }
}

impl pathfinding::num_traits::Zero for Cost {
    fn zero() -> Self {
        Cost(0)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

#[derive(Debug, Clone)]
pub struct PathfinderConfig {
    pub allow_diagonal: bool,
    pub diagonal_cost: f64,
    pub use_terrain_costs: bool,
}

impl Default for PathfinderConfig {
    fn default() -> Self {
        PathfinderConfig {
            allow_diagonal: true,
            diagonal_cost: 2f64.sqrt(),
            use_terrain_costs: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Path {
    pub waypoints: Vec<Vector3>,
    pub total_cost: f64,
    pub steps: usize,
}

impl Path {
    pub fn is_empty(&self) -> bool {
        self.waypoints.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct Pathfinder {
    nav_grid: NavGrid,
    config: PathfinderConfig,
}

impl Pathfinder {
    pub fn new(nav_grid: NavGrid, config: Option<PathfinderConfig>) -> Self {
        Pathfinder {
            nav_grid,
            config: config.unwrap_or_default(),
        }
    }

    pub fn from_lithosphere(litho: &Lithosphere, resolution: Option<f64>) -> Self {
        let nav_grid = NavGrid::from_lithosphere(litho, resolution);
        Self::new(nav_grid, None)
    }

    pub fn find_path(&self, start: Vector3, goal: Vector3) -> Option<Path> {
        let (start_row, start_col) = self.world_to_grid_clamped(start.z, start.x);
        let (goal_row, goal_col) = self.world_to_grid_clamped(goal.z, goal.x);

        if !self.nav_grid.is_passable(start_row, start_col)
            || !self.nav_grid.is_passable(goal_row, goal_col)
        {
            return None;
        }

        let start_pos = GridPos(start_row, start_col);
        let goal_pos = GridPos(goal_row, goal_col);

        let result = astar(
            &start_pos,
            |pos| self.successors(pos),
            |pos| Cost::from_f64(self.heuristic(pos, &goal_pos)),
            |pos| *pos == goal_pos,
        );

        result.map(|(path, cost)| {
            let waypoints: Vec<Vector3> = path
                .iter()
                .map(|GridPos(row, col)| self.nav_grid.grid_to_vec3(*row, *col, None))
                .collect();
            let steps = waypoints.len().saturating_sub(1);
            Path {
                waypoints,
                total_cost: cost.to_f64(),
                steps,
            }
        })
    }

    pub fn find_path_grid(&self, start: GridPos, goal: GridPos) -> Option<Path> {
        let (start_row, start_col) = (
            start.0.min(self.nav_grid.rows.saturating_sub(1)),
            start.1.min(self.nav_grid.cols.saturating_sub(1)),
        );
        let (goal_row, goal_col) = (
            goal.0.min(self.nav_grid.rows.saturating_sub(1)),
            goal.1.min(self.nav_grid.cols.saturating_sub(1)),
        );

        let start_pos = GridPos(start_row, start_col);
        let goal_pos = GridPos(goal_row, goal_col);

        if !self.nav_grid.is_passable(start_row, start_col)
            || !self.nav_grid.is_passable(goal_row, goal_col)
        {
            return None;
        }

        let result = astar(
            &start_pos,
            |pos| self.successors(pos),
            |pos| Cost::from_f64(self.heuristic(pos, &goal_pos)),
            |pos| *pos == goal_pos,
        );

        result.map(|(path, cost)| {
            let waypoints: Vec<Vector3> = path
                .iter()
                .map(|GridPos(row, col)| self.nav_grid.grid_to_vec3(*row, *col, None))
                .collect();
            let steps = waypoints.len().saturating_sub(1);
            Path {
                waypoints,
                total_cost: cost.to_f64(),
                steps,
            }
        })
    }

    fn successors(&self, pos: &GridPos) -> Vec<(GridPos, Cost)> {
        let GridPos(row, col) = *pos;
        let mut successors = Vec::new();

        let directions: Vec<(isize, isize, f64)> = if self.config.allow_diagonal {
            vec![
                (-1, 0, 1.0),
                (1, 0, 1.0),
                (0, -1, 1.0),
                (0, 1, 1.0),
                (-1, -1, self.config.diagonal_cost),
                (-1, 1, self.config.diagonal_cost),
                (1, -1, self.config.diagonal_cost),
                (1, 1, self.config.diagonal_cost),
            ]
        } else {
            vec![(-1, 0, 1.0), (1, 0, 1.0), (0, -1, 1.0), (0, 1, 1.0)]
        };

        for (dr, dc, base_cost) in directions {
            let new_row = row as isize + dr;
            let new_col = col as isize + dc;

            if new_row >= 0
                && new_row < self.nav_grid.rows as isize
                && new_col >= 0
                && new_col < self.nav_grid.cols as isize
            {
                let nr = new_row as usize;
                let nc = new_col as usize;

                if self.nav_grid.is_passable(nr, nc) {
                    let cell = self.nav_grid.cell_at(nr, nc).unwrap();
                    let cost = if self.config.use_terrain_costs {
                        base_cost * cell.movement_cost
                    } else {
                        base_cost
                    };
                    successors.push((GridPos(nr, nc), Cost::from_f64(cost)));
                }
            }
        }

        successors
    }

    fn heuristic(&self, from: &GridPos, to: &GridPos) -> f64 {
        let dr = (from.0 as f64 - to.0 as f64).abs();
        let dc = (from.1 as f64 - to.1 as f64).abs();

        if self.config.allow_diagonal {
            let min_d = dr.min(dc);
            let max_d = dr.max(dc);
            (min_d * self.config.diagonal_cost + (max_d - min_d)) * self.nav_grid.resolution
        } else {
            (dr + dc) * self.nav_grid.resolution
        }
    }

    fn world_to_grid_clamped(&self, lat: f64, lon: f64) -> (usize, usize) {
        let (row, col) = self.nav_grid.world_to_grid(lat, lon);
        (
            row.min(self.nav_grid.rows.saturating_sub(1)),
            col.min(self.nav_grid.cols.saturating_sub(1)),
        )
    }

    pub fn nav_grid(&self) -> &NavGrid {
        &self.nav_grid
    }

    pub fn config(&self) -> &PathfinderConfig {
        &self.config
    }
}

pub fn find_path(
    litho: &Lithosphere,
    start: Vector3,
    goal: Vector3,
    resolution: Option<f64>,
) -> Option<Path> {
    let pathfinder = Pathfinder::from_lithosphere(litho, resolution);
    pathfinder.find_path(start, goal)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pathfinder_creation() {
        let grid = NavGrid::new_flat(20, 20, 5.0);
        let pathfinder = Pathfinder::new(grid, None);
        assert_eq!(pathfinder.nav_grid().rows, 20);
        assert_eq!(pathfinder.nav_grid().cols, 20);
    }

    #[test]
    fn test_straight_line_path() {
        let grid = NavGrid::new_flat(20, 20, 5.0);
        let pathfinder = Pathfinder::new(grid, None);

        let start = Vector3::new(-177.5, 0.0, -87.5);
        let goal = Vector3::new(-177.5, 0.0, -77.5);

        let path = pathfinder.find_path(start, goal);
        assert!(path.is_some());
        let path = path.unwrap();
        assert!(!path.is_empty());
        assert!(path.total_cost > 0.0);
    }

    #[test]
    fn test_path_avoids_impassable() {
        let mut grid = NavGrid::new_flat(10, 10, 5.0);

        for row in 0..5 {
            let idx = grid.index(row, 5);
            grid.cells[idx].is_passable = false;
            grid.cells[idx].movement_cost = f64::INFINITY;
        }

        let config = PathfinderConfig {
            allow_diagonal: false,
            ..Default::default()
        };
        let pathfinder = Pathfinder::new(grid, Some(config));

        let path = pathfinder.find_path_grid(GridPos(0, 0), GridPos(0, 9));
        assert!(
            path.is_some(),
            "Path should go around half-height obstacle at column 5"
        );
    }

    #[test]
    fn test_no_path_when_completely_blocked() {
        let mut grid = NavGrid::new_flat(3, 10, 5.0);

        for col in 0..grid.cols {
            if let Some(cell) = {
                let idx = grid.index(1, col);
                grid.cells.get_mut(idx)
            } {
                cell.is_passable = false;
                cell.movement_cost = f64::INFINITY;
            }
        }

        let config = PathfinderConfig {
            allow_diagonal: false,
            ..Default::default()
        };
        let pathfinder = Pathfinder::new(grid, Some(config));

        let path = pathfinder.find_path_grid(GridPos(0, 5), GridPos(2, 5));
        assert!(path.is_none());
    }

    #[test]
    fn test_path_uses_terrain_costs() {
        let mut grid = NavGrid::new_flat(10, 10, 5.0);

        for i in 0..10 {
            if let Some(cell) = {
                let idx = grid.index(5, i);
                grid.cells.get_mut(idx)
            } {
                cell.movement_cost = 100.0;
            }
        }

        let pathfinder_with_costs = Pathfinder::new(
            grid.clone(),
            Some(PathfinderConfig {
                use_terrain_costs: true,
                ..Default::default()
            }),
        );

        let pathfinder_no_costs = Pathfinder::new(
            grid.clone(),
            Some(PathfinderConfig {
                use_terrain_costs: false,
                ..Default::default()
            }),
        );

        let start = GridPos(0, 5);
        let goal = GridPos(9, 5);

        let path_with_costs = pathfinder_with_costs.find_path_grid(start, goal);
        let path_no_costs = pathfinder_no_costs.find_path_grid(start, goal);

        assert!(path_with_costs.is_some());
        assert!(path_no_costs.is_some());

        let path_with = path_with_costs.unwrap();
        let path_without = path_no_costs.unwrap();

        if path_with.steps != path_without.steps {
            assert!(
                path_with.total_cost <= path_without.total_cost,
                "Terrain-aware path should have lower or equal cost"
            );
        }
    }

    #[test]
    fn test_diagonal_movement() {
        let grid = NavGrid::new_flat(10, 10, 5.0);
        let pathfinder = Pathfinder::new(
            grid,
            Some(PathfinderConfig {
                allow_diagonal: true,
                ..Default::default()
            }),
        );

        let start = GridPos(0, 0);
        let goal = GridPos(5, 5);

        let path = pathfinder.find_path_grid(start, goal);
        assert!(path.is_some());
        assert!(path.unwrap().steps <= 6);
    }

    #[test]
    fn test_find_path_function() {
        let litho = Lithosphere::new();
        let start = Vector3::new(0.0, 0.0, 45.0);
        let goal = Vector3::new(10.0, 0.0, 45.0);

        let path = find_path(&litho, start, goal, Some(5.0));
        assert!(path.is_some());
    }

    #[test]
    fn test_path_impassable_start_or_goal() {
        let mut grid = NavGrid::new_flat(5, 5, 5.0);

        if let Some(cell) = {
            let idx = grid.index(0, 0);
            grid.cells.get_mut(idx)
        } {
            cell.is_passable = false;
            cell.movement_cost = f64::INFINITY;
        }

        let pathfinder = Pathfinder::new(grid, None);
        let start = Vector3::new(-180.0, 0.0, -90.0);
        let goal = Vector3::new(-170.0, 0.0, -80.0);

        let path = pathfinder.find_path(start, goal);
        assert!(path.is_none());
    }

    #[test]
    fn test_path_result_properties() {
        let grid = NavGrid::new_flat(10, 10, 5.0);
        let pathfinder = Pathfinder::new(grid, None);

        let path = pathfinder.find_path_grid(GridPos(0, 0), GridPos(5, 5));
        assert!(path.is_some());

        let path = path.unwrap();
        assert!(!path.is_empty());
        assert_eq!(path.waypoints.len(), path.steps + 1);
        assert!(path.total_cost > 0.0);
    }

    #[test]
    fn test_cost_arithmetic() {
        let c1 = Cost::from_f64(1.5);
        let c2 = Cost::from_f64(2.5);
        let sum = c1 + c2;
        assert!((sum.to_f64() - 4.0).abs() < 0.002);
    }
}
