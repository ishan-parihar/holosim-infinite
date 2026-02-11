use crate::entity_layer7::layer7::EntityId;
use crate::gui::camera::Camera2D;
use nalgebra_glm::Vec3;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn from_center_half_size(center: Vec3, half_size: f32) -> Self {
        Self {
            min: center - Vec3::new(half_size, half_size, half_size),
            max: center + Vec3::new(half_size, half_size, half_size),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OctreeNode {
    center: Vec3,
    half_size: f32,
    entities: Vec<(EntityId, Vec3)>,
    children: Option<Box<[OctreeNode; 8]>>,
}

impl OctreeNode {
    fn new(center: Vec3, half_size: f32) -> Self {
        Self {
            center,
            half_size,
            entities: Vec::new(),
            children: None,
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.is_none()
    }

    fn max_entities() -> usize {
        32
    }

    fn max_depth() -> usize {
        8
    }

    fn insert(&mut self, entity_id: EntityId, position: Vec3, depth: usize) -> bool {
        if !self.contains(position) {
            return false;
        }

        if self.is_leaf() {
            if self.entities.len() < Self::max_entities() || depth >= Self::max_depth() {
                self.entities.push((entity_id, position));
                return true;
            }

            self.subdivide();
        }

        for child in self.children.as_mut().unwrap().iter_mut() {
            let result = child.insert(entity_id.clone(), position, depth + 1);
            if result {
                return true;
            }
        }

        false
    }

    fn contains(&self, position: Vec3) -> bool {
        position.x >= self.center.x - self.half_size
            && position.x <= self.center.x + self.half_size
            && position.y >= self.center.y - self.half_size
            && position.y <= self.center.y + self.half_size
            && position.z >= self.center.z - self.half_size
            && position.z <= self.center.z + self.half_size
    }

    fn subdivide(&mut self) {
        if self.children.is_some() {
            return;
        }

        let half = self.half_size * 0.5;
        let quarter = self.half_size * 0.25;

        let mut children = [
            OctreeNode::new(
                Vec3::new(
                    self.center.x - quarter,
                    self.center.y - quarter,
                    self.center.z - quarter,
                ),
                half,
            ),
            OctreeNode::new(
                Vec3::new(
                    self.center.x + quarter,
                    self.center.y - quarter,
                    self.center.z - quarter,
                ),
                half,
            ),
            OctreeNode::new(
                Vec3::new(
                    self.center.x - quarter,
                    self.center.y + quarter,
                    self.center.z - quarter,
                ),
                half,
            ),
            OctreeNode::new(
                Vec3::new(
                    self.center.x + quarter,
                    self.center.y + quarter,
                    self.center.z - quarter,
                ),
                half,
            ),
            OctreeNode::new(
                Vec3::new(
                    self.center.x - quarter,
                    self.center.y - quarter,
                    self.center.z + quarter,
                ),
                half,
            ),
            OctreeNode::new(
                Vec3::new(
                    self.center.x + quarter,
                    self.center.y - quarter,
                    self.center.z + quarter,
                ),
                half,
            ),
            OctreeNode::new(
                Vec3::new(
                    self.center.x - quarter,
                    self.center.y + quarter,
                    self.center.z + quarter,
                ),
                half,
            ),
            OctreeNode::new(
                Vec3::new(
                    self.center.x + quarter,
                    self.center.y + quarter,
                    self.center.z + quarter,
                ),
                half,
            ),
        ];

        for (entity_id, position) in self.entities.drain(..) {
            for child in &mut children {
                if child.contains(position) {
                    child.insert(entity_id, position, 1);
                    break;
                }
            }
        }

        self.children = Some(Box::new(children));
    }

    fn query(&self, aabb: AABB, results: &mut Vec<(EntityId, Vec3)>) {
        if !self.intersects_aabb(aabb) {
            return;
        }

        if self.is_leaf() {
            results.extend(
                self.entities
                    .iter()
                    .filter(|(_, pos)| {
                        pos.x >= aabb.min.x
                            && pos.x <= aabb.max.x
                            && pos.y >= aabb.min.y
                            && pos.y <= aabb.max.y
                            && pos.z >= aabb.min.z
                            && pos.z <= aabb.max.z
                    })
                    .map(|(id, pos)| (id.clone(), *pos)),
            );
        } else {
            for child in self.children.as_ref().unwrap().iter() {
                child.query(aabb, results);
            }
        }
    }

    fn intersects_aabb(&self, aabb: AABB) -> bool {
        let node_min = Vec3::new(
            self.center.x - self.half_size,
            self.center.y - self.half_size,
            self.center.z - self.half_size,
        );
        let node_max = Vec3::new(
            self.center.x + self.half_size,
            self.center.y + self.half_size,
            self.center.z + self.half_size,
        );

        node_min.x <= aabb.max.x
            && node_max.x >= aabb.min.x
            && node_min.y <= aabb.max.y
            && node_max.y >= aabb.min.y
            && node_min.z <= aabb.max.z
            && node_max.z >= aabb.min.z
    }

    fn count_entities(&self) -> usize {
        if self.is_leaf() {
            self.entities.len()
        } else {
            self.children
                .as_ref()
                .unwrap()
                .iter()
                .map(|c| c.count_entities())
                .sum()
        }
    }
}

pub struct Octree {
    root: OctreeNode,
    entity_positions: HashMap<EntityId, Vec3>,
}

impl Octree {
    pub fn new(center: Vec3, size: f32) -> Self {
        Self {
            root: OctreeNode::new(center, size * 0.5),
            entity_positions: HashMap::new(),
        }
    }

    pub fn insert(&mut self, entity_id: EntityId, position: Vec3) -> bool {
        self.entity_positions.insert(entity_id.clone(), position);
        self.root.insert(entity_id, position, 0)
    }

    pub fn remove(&mut self, entity_id: &EntityId) {
        self.entity_positions.remove(entity_id);
    }

    pub fn update(&mut self, entity_id: EntityId, new_position: Vec3) {
        self.remove(&entity_id);
        self.insert(entity_id, new_position);
    }

    pub fn query(&self, aabb: AABB) -> Vec<(EntityId, Vec3)> {
        let mut results = Vec::new();
        self.root.query(aabb, &mut results);
        results
    }

    pub fn query_radius(&self, center: Vec3, radius: f32) -> Vec<(EntityId, Vec3)> {
        let aabb = AABB::new(
            Vec3::new(center.x - radius, center.y - radius, center.z - radius),
            Vec3::new(center.x + radius, center.y + radius, center.z + radius),
        );
        self.query(aabb)
            .into_iter()
            .filter(|(_, pos)| nalgebra_glm::distance(pos, &center) <= radius)
            .collect()
    }

    pub fn count_entities(&self) -> usize {
        self.root.count_entities()
    }

    pub fn rebuild(&mut self, center: Vec3, size: f32) {
        let positions = self.entity_positions.clone();
        *self = Self::new(center, size);
        for (id, pos) in positions {
            self.insert(id, pos);
        }
    }
}

pub struct FrustumCuller {
    planes: [nalgebra_glm::Vec4; 6],
}

impl FrustumCuller {
    pub fn from_camera(camera: &Camera2D, aspect_ratio: f32) -> Self {
        let fov = std::f32::consts::PI / 4.0;
        let near = 0.1;
        let far = 1000.0;

        let projection = nalgebra_glm::perspective(fov, aspect_ratio, near, far);
        let view = camera.view_matrix();
        let view_projection = projection * view;

        let mut planes = [nalgebra_glm::Vec4::zeros(); 6];

        for i in 0..4 {
            let row = view_projection.row(i);
            planes[0][i] = row[3] + row[0]; // Left
            planes[1][i] = row[3] - row[0]; // Right
            planes[2][i] = row[3] + row[1]; // Bottom
            planes[3][i] = row[3] - row[1]; // Top
            planes[4][i] = row[3] + row[2]; // Near
            planes[5][i] = row[3] - row[2]; // Far
        }

        for plane in &mut planes {
            let length = (plane[0].powi(2) + plane[1].powi(2) + plane[2].powi(2)).sqrt();
            if length > 0.0 {
                *plane = *plane / length;
            }
        }

        Self { planes }
    }

    pub fn is_visible(&self, center: Vec3, radius: f32) -> bool {
        for plane in &self.planes {
            let distance =
                plane[0] * center.x + plane[1] * center.y + plane[2] * center.z + plane[3];
            if distance < -radius {
                return false;
            }
        }
        true
    }

    pub fn is_aabb_visible(&self, aabb: AABB) -> bool {
        let center = (aabb.min + aabb.max) * 0.5;
        let extents = (aabb.max - aabb.min) * 0.5;
        let radius = extents.x.max(extents.y).max(extents.z);
        self.is_visible(center, radius)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_octree_insert() {
        let mut octree = Octree::new(Vec3::zeros(), 100.0);
        let entity_id = EntityId::new("test".to_string());
        assert!(octree.insert(entity_id, Vec3::new(10.0, 10.0, 10.0)));
        assert_eq!(octree.count_entities(), 1);
    }

    #[test]
    fn test_octree_query() {
        let mut octree = Octree::new(Vec3::zeros(), 100.0);
        let id1 = EntityId::new("test1".to_string());
        let id2 = EntityId::new("test2".to_string());
        octree.insert(id1.clone(), Vec3::new(10.0, 10.0, 10.0));
        octree.insert(id2, Vec3::new(50.0, 50.0, 50.0));

        let results = octree.query_radius(Vec3::new(10.0, 10.0, 10.0), 20.0);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, id1);
    }
}
