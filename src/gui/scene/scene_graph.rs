use crate::entity_layer7::layer7::EntityId;
use crate::gui::camera::Camera2D;
use nalgebra_glm::{rotation, scaling, translation, Mat4, Quat, Vec3};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

static NODE_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(u64);

impl NodeId {
    pub fn new() -> Self {
        Self(NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst))
    }

    pub fn from_raw(id: u64) -> Self {
        Self(id)
    }

    pub fn as_raw(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec3::zeros(),
            rotation: Quat::identity(),
            scale: Vec3::new(1.0, 1.0, 1.0),
        }
    }
}

impl Transform {
    pub fn to_matrix(&self) -> Mat4 {
        let t = translation(&self.position);
        let r = nalgebra_glm::quat_to_mat4(&self.rotation);
        let s = scaling(&self.scale);
        t * r * s
    }

    pub fn translation(position: Vec3) -> Self {
        Self {
            position,
            ..Default::default()
        }
    }

    pub fn scale(scale: Vec3) -> Self {
        Self {
            scale,
            ..Default::default()
        }
    }

    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        Self {
            position: eye,
            rotation: Quat::identity(),
            scale: Vec3::new(1.0, 1.0, 1.0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SceneNode {
    pub id: NodeId,
    pub name: String,
    pub transform: Transform,
    pub world_transform: Mat4,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub entity_id: Option<EntityId>,
    pub visible: bool,
    pub static_node: bool,
}

impl SceneNode {
    pub fn new(name: impl Into<String>) -> Self {
        let id = NodeId::new();
        Self {
            id,
            name: name.into(),
            transform: Transform::default(),
            world_transform: Mat4::identity(),
            parent: None,
            children: Vec::new(),
            entity_id: None,
            visible: true,
            static_node: false,
        }
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    pub fn with_entity(mut self, entity_id: EntityId) -> Self {
        self.entity_id = Some(entity_id);
        self
    }

    pub fn with_parent(mut self, parent: NodeId) -> Self {
        self.parent = Some(parent);
        self
    }

    pub fn with_visibility(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn is_static(&self) -> bool {
        self.static_node
    }

    pub fn add_child(&mut self, child: NodeId) {
        if !self.children.contains(&child) {
            self.children.push(child);
        }
    }

    pub fn remove_child(&mut self, child: NodeId) {
        self.children.retain(|&id| id != child);
    }
}

pub struct SceneGraph {
    nodes: HashMap<NodeId, SceneNode>,
    root_nodes: Vec<NodeId>,
    dirty_nodes: Vec<NodeId>,
}

impl Default for SceneGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl SceneGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            root_nodes: Vec::new(),
            dirty_nodes: Vec::new(),
        }
    }

    pub fn create_node(&mut self, name: impl Into<String>) -> NodeId {
        let node = SceneNode::new(name);
        let id = node.id;
        self.root_nodes.push(id);
        self.nodes.insert(id, node);
        id
    }

    pub fn create_node_with_transform(
        &mut self,
        name: impl Into<String>,
        transform: Transform,
    ) -> NodeId {
        let node = SceneNode::new(name).with_transform(transform);
        let id = node.id;
        self.root_nodes.push(id);
        self.nodes.insert(id, node);
        id
    }

    pub fn create_entity_node(&mut self, name: impl Into<String>, entity_id: EntityId) -> NodeId {
        let node = SceneNode::new(name).with_entity(entity_id);
        let id = node.id;
        self.root_nodes.push(id);
        self.nodes.insert(id, node);
        id
    }

    pub fn add_child(&mut self, parent: NodeId, child: NodeId) -> bool {
        let parent_exists = self.nodes.contains_key(&parent);
        let child_exists = self.nodes.contains_key(&child);

        if !parent_exists || !child_exists {
            return false;
        }

        let child_has_parent = self
            .nodes
            .get(&child)
            .map(|n| n.parent.is_some())
            .unwrap_or(false);
        if child_has_parent {
            return false;
        }

        if let Some(parent_node) = self.nodes.get_mut(&parent) {
            parent_node.add_child(child);
        }

        if let Some(child_node) = self.nodes.get_mut(&child) {
            child_node.parent = Some(parent);
        }

        self.root_nodes.retain(|&id| id != child);
        self.mark_dirty(child);
        true
    }

    pub fn remove_child(&mut self, parent: NodeId, child: NodeId) -> bool {
        let parent_exists = self.nodes.contains_key(&parent);
        let child_exists = self.nodes.contains_key(&child);

        if !parent_exists || !child_exists {
            return false;
        }

        if let Some(parent_node) = self.nodes.get_mut(&parent) {
            parent_node.remove_child(child);
        }

        if let Some(child_node) = self.nodes.get_mut(&child) {
            child_node.parent = None;
        }

        self.root_nodes.push(child);
        self.mark_dirty(child);
        true
    }

    pub fn reparent(&mut self, child: NodeId, new_parent: NodeId) -> bool {
        if let Some(child_node) = self.nodes.get(&child) {
            if let Some(old_parent) = child_node.parent {
                self.remove_child(old_parent, child);
            }
        }
        self.add_child(new_parent, child)
    }

    pub fn remove_node(&mut self, node_id: NodeId) -> bool {
        if let Some(node) = self.nodes.remove(&node_id) {
            if let Some(parent) = node.parent {
                if let Some(parent_node) = self.nodes.get_mut(&parent) {
                    parent_node.remove_child(node_id);
                }
            } else {
                self.root_nodes.retain(|&id| id != node_id);
            }

            for child_id in node.children {
                self.root_nodes.push(child_id);
                if let Some(child_node) = self.nodes.get_mut(&child_id) {
                    child_node.parent = None;
                }
            }

            self.dirty_nodes.retain(|&id| id != node_id);
            true
        } else {
            false
        }
    }

    pub fn get_node(&self, node_id: NodeId) -> Option<&SceneNode> {
        self.nodes.get(&node_id)
    }

    pub fn get_node_mut(&mut self, node_id: NodeId) -> Option<&mut SceneNode> {
        self.nodes.get_mut(&node_id)
    }

    pub fn set_transform(&mut self, node_id: NodeId, transform: Transform) -> bool {
        if let Some(node) = self.nodes.get_mut(&node_id) {
            node.transform = transform;
            self.mark_dirty(node_id);
            true
        } else {
            false
        }
    }

    pub fn set_position(&mut self, node_id: NodeId, position: Vec3) -> bool {
        if let Some(node) = self.nodes.get_mut(&node_id) {
            node.transform.position = position;
            self.mark_dirty(node_id);
            true
        } else {
            false
        }
    }

    pub fn set_rotation(&mut self, node_id: NodeId, rotation: Quat) -> bool {
        if let Some(node) = self.nodes.get_mut(&node_id) {
            node.transform.rotation = rotation;
            self.mark_dirty(node_id);
            true
        } else {
            false
        }
    }

    pub fn set_scale(&mut self, node_id: NodeId, scale: Vec3) -> bool {
        if let Some(node) = self.nodes.get_mut(&node_id) {
            node.transform.scale = scale;
            self.mark_dirty(node_id);
            true
        } else {
            false
        }
    }

    pub fn set_visibility(&mut self, node_id: NodeId, visible: bool) -> bool {
        if let Some(node) = self.nodes.get_mut(&node_id) {
            node.visible = visible;
            true
        } else {
            false
        }
    }

    pub fn mark_dirty(&mut self, node_id: NodeId) {
        if !self.dirty_nodes.contains(&node_id) {
            self.dirty_nodes.push(node_id);
        }
    }

    pub fn update_transforms(&mut self) {
        while let Some(node_id) = self.dirty_nodes.pop() {
            self.update_node_transform(node_id);
        }
    }

    fn update_node_transform(&mut self, node_id: NodeId) {
        let (local_transform, parent_transform, children) =
            if let Some(node) = self.nodes.get(&node_id) {
                let local_transform = node.transform.to_matrix();
                let parent_transform = if let Some(parent_id) = node.parent {
                    if let Some(parent) = self.nodes.get(&parent_id) {
                        parent.world_transform
                    } else {
                        Mat4::identity()
                    }
                } else {
                    Mat4::identity()
                };
                let children = node.children.clone();
                (local_transform, parent_transform, children)
            } else {
                return;
            };

        let world_transform = parent_transform * local_transform;

        if let Some(node) = self.nodes.get_mut(&node_id) {
            node.world_transform = world_transform;
        }

        for child_id in children {
            if !self.dirty_nodes.contains(&child_id) {
                self.dirty_nodes.push(child_id);
            }
        }
    }

    pub fn get_world_transform(&self, node_id: NodeId) -> Option<Mat4> {
        self.nodes.get(&node_id).map(|node| node.world_transform)
    }

    pub fn get_world_position(&self, node_id: NodeId) -> Option<Vec3> {
        self.nodes.get(&node_id).map(|node| {
            Vec3::new(
                node.world_transform[12],
                node.world_transform[13],
                node.world_transform[14],
            )
        })
    }

    pub fn get_all_nodes(&self) -> impl Iterator<Item = &SceneNode> {
        self.nodes.values()
    }

    pub fn get_visible_nodes(&self) -> Vec<&SceneNode> {
        self.nodes.values().filter(|node| node.visible).collect()
    }

    pub fn get_root_nodes(&self) -> Vec<&SceneNode> {
        self.root_nodes
            .iter()
            .filter_map(|id| self.nodes.get(id))
            .collect()
    }

    pub fn get_children(&self, node_id: NodeId) -> Vec<&SceneNode> {
        if let Some(node) = self.nodes.get(&node_id) {
            node.children
                .iter()
                .filter_map(|id| self.nodes.get(id))
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn find_nodes_by_name(&self, name: &str) -> Vec<&SceneNode> {
        self.nodes
            .values()
            .filter(|node| node.name == name)
            .collect()
    }

    pub fn find_node_by_entity(&self, entity_id: &EntityId) -> Option<&SceneNode> {
        self.nodes
            .values()
            .find(|node| node.entity_id.as_ref() == Some(entity_id))
    }

    pub fn find_node_by_entity_mut(&mut self, entity_id: &EntityId) -> Option<&mut SceneNode> {
        self.nodes
            .values_mut()
            .find(|node| node.entity_id.as_ref() == Some(entity_id))
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.root_nodes.clear();
        self.dirty_nodes.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let graph = SceneGraph::new();
        assert_eq!(graph.node_count(), 0);
    }

    #[test]
    fn test_add_node() {
        let mut graph = SceneGraph::new();
        let id = graph.create_node("TestNode");
        assert_eq!(graph.node_count(), 1);
        assert!(graph.get_node(id).is_some());
    }

    #[test]
    fn test_hierarchy() {
        let mut graph = SceneGraph::new();
        let parent = graph.create_node("Parent");
        let child = graph.create_node("Child");
        assert!(graph.add_child(parent, child));
        assert_eq!(graph.get_children(parent).len(), 1);
    }
}
