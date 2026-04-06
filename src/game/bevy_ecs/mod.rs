use bevy_ecs::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn distance_squared(&self, other: &Position) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }

    pub fn distance(&self, other: &Position) -> f64 {
        self.distance_squared(other).sqrt()
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::zero()
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Velocity {
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
}

impl Velocity {
    pub fn new(vx: f64, vy: f64, vz: f64) -> Self {
        Self { vx, vy, vz }
    }

    pub fn zero() -> Self {
        Self {
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
        }
    }

    pub fn speed(&self) -> f64 {
        (self.vx * self.vx + self.vy * self.vy + self.vz * self.vz).sqrt()
    }
}

impl Default for Velocity {
    fn default() -> Self {
        Self::zero()
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Health {
    pub current: f64,
    pub max: f64,
}

impl Health {
    pub fn new(current: f64, max: f64) -> Self {
        Self {
            current: current.clamp(0.0, max),
            max,
        }
    }

    pub fn full(max: f64) -> Self {
        Self::new(max, max)
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }

    pub fn ratio(&self) -> f64 {
        if self.max <= 0.0 {
            0.0
        } else {
            self.current / self.max
        }
    }
}

impl Default for Health {
    fn default() -> Self {
        Self::full(100.0)
    }
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Item {
    pub id: u64,
    pub name: String,
    pub quantity: u32,
}

impl Item {
    pub fn new(id: u64, name: &str, quantity: u32) -> Self {
        Self {
            id,
            name: name.to_string(),
            quantity,
        }
    }
}

#[derive(Component, Debug, Clone, Default)]
pub struct Inventory {
    pub items: Vec<Item>,
    pub capacity: usize,
}

impl Inventory {
    pub fn new(capacity: usize) -> Self {
        Self {
            items: Vec::new(),
            capacity,
        }
    }

    pub fn is_full(&self) -> bool {
        self.items.len() >= self.capacity
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn add_item(&mut self, item: Item) -> bool {
        if self.is_full() {
            return false;
        }
        self.items.push(item);
        true
    }

    pub fn remove_item_by_id(&mut self, id: u64) -> Option<Item> {
        if let Some(pos) = self.items.iter().position(|i| i.id == id) {
            Some(self.items.remove(pos))
        } else {
            None
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityRef {
    pub id: u64,
}

impl EntityRef {
    pub fn new(id: u64) -> Self {
        Self { id }
    }
}

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct Name(pub String);

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Active;

pub struct EcsWorld {
    world: World,
}

impl EcsWorld {
    pub fn new() -> Self {
        Self {
            world: World::new(),
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn spawn_ref(&mut self, entity_id: u64) -> Entity {
        self.world.spawn(EntityRef::new(entity_id)).id()
    }

    pub fn spawn_at(&mut self, entity_id: u64, x: f64, y: f64, z: f64) -> Entity {
        self.world
            .spawn((EntityRef::new(entity_id), Position::new(x, y, z)))
            .id()
    }

    pub fn spawn_moving(&mut self, entity_id: u64, pos: Position, vel: Velocity) -> Entity {
        self.world.spawn((EntityRef::new(entity_id), pos, vel)).id()
    }

    pub fn spawn_full(
        &mut self,
        entity_id: u64,
        position: Position,
        velocity: Velocity,
        health: Health,
        inventory: Inventory,
    ) -> Entity {
        self.world
            .spawn((
                EntityRef::new(entity_id),
                position,
                velocity,
                health,
                inventory,
                Active,
            ))
            .id()
    }

    pub fn despawn(&mut self, entity: Entity) {
        self.world.despawn(entity);
    }

    pub fn despawn_by_ref(&mut self, entity_id: u64) {
        if let Some(e) = self.find_entity_by_ref(entity_id) {
            self.world.despawn(e);
        }
    }

    pub fn entity_count(&mut self) -> usize {
        let mut q = self.world.query::<()>();
        q.iter(&self.world).count()
    }

    pub fn has_entity_ref(&mut self, entity_id: u64) -> bool {
        self.find_entity_by_ref(entity_id).is_some()
    }

    pub fn entities_with_position(&mut self) -> Vec<(u64, Position)> {
        let mut results = Vec::new();
        let mut query = self.world.query::<(&EntityRef, &Position)>();
        for (entity_ref, pos) in query.iter(&self.world) {
            results.push((entity_ref.id, *pos));
        }
        results
    }

    pub fn alive_entities(&mut self) -> Vec<(u64, Health)> {
        let mut results = Vec::new();
        let mut query = self.world.query::<(&EntityRef, &Health)>();
        for (entity_ref, health) in query.iter(&self.world) {
            if health.is_alive() {
                results.push((entity_ref.id, *health));
            }
        }
        results
    }

    pub fn damaged_entities(&mut self, threshold_ratio: f64) -> Vec<(u64, Health)> {
        let mut results = Vec::new();
        let mut query = self.world.query::<(&EntityRef, &Health)>();
        for (entity_ref, health) in query.iter(&self.world) {
            if health.ratio() < threshold_ratio {
                results.push((entity_ref.id, *health));
            }
        }
        results
    }

    pub fn entities_in_radius(&mut self, center: Position, radius: f64) -> Vec<(u64, Position)> {
        let radius_sq = radius * radius;
        let mut results = Vec::new();
        let mut query = self.world.query::<(&EntityRef, &Position)>();
        for (entity_ref, pos) in query.iter(&self.world) {
            if pos.distance_squared(&center) <= radius_sq {
                results.push((entity_ref.id, *pos));
            }
        }
        results
    }

    pub fn get_position(&mut self, entity_id: u64) -> Option<Position> {
        let mut query = self.world.query::<(&EntityRef, &Position)>();
        for (entity_ref, pos) in query.iter(&self.world) {
            if entity_ref.id == entity_id {
                return Some(*pos);
            }
        }
        None
    }

    pub fn get_velocity(&mut self, entity_id: u64) -> Option<Velocity> {
        let mut query = self.world.query::<(&EntityRef, &Velocity)>();
        for (entity_ref, vel) in query.iter(&self.world) {
            if entity_ref.id == entity_id {
                return Some(*vel);
            }
        }
        None
    }

    pub fn set_position(&mut self, entity_id: u64, new_pos: Position) -> bool {
        let mut query = self.world.query::<(&EntityRef, &mut Position)>();
        for (entity_ref, mut pos) in query.iter_mut(&mut self.world) {
            if entity_ref.id == entity_id {
                *pos = new_pos;
                return true;
            }
        }
        false
    }

    pub fn set_velocity(&mut self, entity_id: u64, new_vel: Velocity) -> bool {
        let mut query = self.world.query::<(&EntityRef, &mut Velocity)>();
        for (entity_ref, mut vel) in query.iter_mut(&mut self.world) {
            if entity_ref.id == entity_id {
                *vel = new_vel;
                return true;
            }
        }
        false
    }

    pub fn damage_entity(&mut self, entity_id: u64, amount: f64) -> bool {
        let mut query = self.world.query::<(&EntityRef, &mut Health)>();
        for (entity_ref, mut health) in query.iter_mut(&mut self.world) {
            if entity_ref.id == entity_id {
                health.current = (health.current - amount).max(0.0);
                return true;
            }
        }
        false
    }

    pub fn heal_entity(&mut self, entity_id: u64, amount: f64) -> bool {
        let mut query = self.world.query::<(&EntityRef, &mut Health)>();
        for (entity_ref, mut health) in query.iter_mut(&mut self.world) {
            if entity_ref.id == entity_id {
                health.current = (health.current + amount).min(health.max);
                return true;
            }
        }
        false
    }

    fn find_entity_by_ref(&mut self, entity_id: u64) -> Option<Entity> {
        let mut q = self.world.query::<(Entity, &EntityRef)>();
        for (entity, eref) in q.iter(&self.world) {
            if eref.id == entity_id {
                return Some(entity);
            }
        }
        None
    }
}

impl Default for EcsWorld {
    fn default() -> Self {
        Self::new()
    }
}

pub fn movement_system(world: &mut World, dt: f64) {
    let mut query = world.query::<(&Velocity, &mut Position)>();
    for (vel, mut pos) in query.iter_mut(world) {
        pos.x += vel.vx * dt;
        pos.y += vel.vy * dt;
        pos.z += vel.vz * dt;
    }
}

pub fn collision_system(world: &mut World, threshold: f64) -> Vec<(u64, u64, f64)> {
    let threshold_sq = threshold * threshold;
    let mut results = Vec::new();

    let mut q = world.query::<(&EntityRef, &Position)>();
    let positions: Vec<(u64, Position)> =
        q.iter(world).map(|(eref, pos)| (eref.id, *pos)).collect();

    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            let dist_sq = positions[i].1.distance_squared(&positions[j].1);
            if dist_sq <= threshold_sq {
                results.push((positions[i].0, positions[j].0, dist_sq.sqrt()));
            }
        }
    }

    results
}

pub fn decay_system(world: &mut World, rate: f64, dt: f64) {
    let mut query = world.query::<&mut Health>();
    for mut health in query.iter_mut(world) {
        if health.is_alive() {
            health.current = (health.current - rate * dt).max(0.0);
        }
    }
}

pub fn inventory_cleanup_system(world: &mut World) {
    let mut query = world.query::<&mut Inventory>();
    for mut inv in query.iter_mut(world) {
        inv.items.retain(|item| item.quantity > 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_creation_and_defaults() {
        let pos = Position::new(1.0, 2.0, 3.0);
        assert_eq!(pos.x, 1.0);
        assert_eq!(pos.y, 2.0);
        assert_eq!(pos.z, 3.0);

        let vel = Velocity::new(0.5, -0.5, 0.0);
        assert!((vel.speed() - 0.7071067811865476).abs() < 1e-10);

        let health = Health::full(100.0);
        assert!(health.is_alive());
        assert!((health.ratio() - 1.0).abs() < f64::EPSILON);

        let health_damaged = Health::new(30.0, 100.0);
        assert!((health_damaged.ratio() - 0.3).abs() < f64::EPSILON);

        let inv = Inventory::new(10);
        assert!(inv.is_empty());
        assert!(!inv.is_full());

        let entity_ref = EntityRef::new(42);
        assert_eq!(entity_ref.id, 42);
    }

    #[test]
    fn test_spawn_and_despawn() {
        let mut ecs = EcsWorld::new();
        let e1 = ecs.spawn_at(1, 0.0, 0.0, 0.0);
        let e2 = ecs.spawn_at(2, 10.0, 10.0, 10.0);

        assert_eq!(ecs.entity_count(), 2);
        assert!(ecs.has_entity_ref(1));
        assert!(ecs.has_entity_ref(2));
        assert!(!ecs.has_entity_ref(999));

        ecs.despawn(e1);
        assert_eq!(ecs.entity_count(), 1);
        assert!(!ecs.has_entity_ref(1));
        assert!(ecs.has_entity_ref(2));

        ecs.despawn_by_ref(2);
        assert_eq!(ecs.entity_count(), 0);
    }

    #[test]
    fn test_spawn_full_entity() {
        let mut ecs = EcsWorld::new();
        let inv = Inventory::new(5);
        let _e = ecs.spawn_full(
            100,
            Position::new(1.0, 2.0, 3.0),
            Velocity::new(0.1, 0.2, 0.3),
            Health::full(100.0),
            inv,
        );

        assert_eq!(ecs.entity_count(), 1);
        let pos = ecs.get_position(100).unwrap();
        assert!((pos.x - 1.0).abs() < f64::EPSILON);
        let vel = ecs.get_velocity(100).unwrap();
        assert!((vel.vx - 0.1).abs() < f64::EPSILON);
    }

    #[test]
    fn test_query_entities_with_position() {
        let mut ecs = EcsWorld::new();
        ecs.spawn_at(1, 0.0, 0.0, 0.0);
        ecs.spawn_at(2, 5.0, 5.0, 5.0);
        ecs.spawn_ref(3);

        let with_pos = ecs.entities_with_position();
        assert_eq!(with_pos.len(), 2);

        let ids: Vec<u64> = with_pos.iter().map(|(id, _)| *id).collect();
        assert!(ids.contains(&1));
        assert!(ids.contains(&2));
        assert!(!ids.contains(&3));
    }

    #[test]
    fn test_damaged_and_alive_queries() {
        let mut ecs = EcsWorld::new();
        ecs.world_mut()
            .spawn((EntityRef::new(1), Health::full(100.0)));
        ecs.world_mut()
            .spawn((EntityRef::new(2), Health::new(50.0, 100.0)));
        ecs.world_mut()
            .spawn((EntityRef::new(3), Health::new(0.0, 100.0)));

        let alive = ecs.alive_entities();
        assert_eq!(alive.len(), 2);

        let damaged = ecs.damaged_entities(0.6);
        assert_eq!(damaged.len(), 2);
        let damaged_ids: Vec<u64> = damaged.iter().map(|(id, _)| *id).collect();
        assert!(damaged_ids.contains(&2));
        assert!(damaged_ids.contains(&3));
    }

    #[test]
    fn test_entities_in_radius() {
        let mut ecs = EcsWorld::new();
        ecs.spawn_at(1, 0.0, 0.0, 0.0);
        ecs.spawn_at(2, 1.0, 0.0, 0.0);
        ecs.spawn_at(3, 3.0, 0.0, 0.0);
        ecs.spawn_at(4, 10.0, 10.0, 10.0);

        let nearby = ecs.entities_in_radius(Position::zero(), 2.0);
        assert_eq!(nearby.len(), 2);

        let far = ecs.entities_in_radius(Position::zero(), 0.5);
        assert_eq!(far.len(), 1);
    }

    #[test]
    fn test_movement_system() {
        let mut ecs = EcsWorld::new();
        ecs.spawn_moving(1, Position::zero(), Velocity::new(1.0, 2.0, 3.0));

        movement_system(ecs.world_mut(), 0.5);

        let pos = ecs.get_position(1).unwrap();
        assert!((pos.x - 0.5).abs() < f64::EPSILON);
        assert!((pos.y - 1.0).abs() < f64::EPSILON);
        assert!((pos.z - 1.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_collision_system() {
        let mut ecs = EcsWorld::new();
        ecs.spawn_at(1, 0.0, 0.0, 0.0);
        ecs.spawn_at(2, 0.5, 0.0, 0.0);
        ecs.spawn_at(3, 10.0, 10.0, 10.0);

        let collisions = collision_system(ecs.world_mut(), 1.0);
        assert_eq!(collisions.len(), 1);
        assert!((collisions[0].2 - 0.5).abs() < f64::EPSILON);

        let no_collisions = collision_system(ecs.world_mut(), 0.1);
        assert!(no_collisions.is_empty());
    }

    #[test]
    fn test_damage_and_heal() {
        let mut ecs = EcsWorld::new();
        ecs.world_mut()
            .spawn((EntityRef::new(1), Health::full(100.0)));

        assert!(ecs.damage_entity(1, 30.0));
        {
            let mut q = ecs.world_mut().query::<&Health>();
            let h = q.iter(ecs.world()).next().unwrap();
            assert!((h.current - 70.0).abs() < f64::EPSILON);
        }

        assert!(ecs.heal_entity(1, 20.0));
        {
            let mut q = ecs.world_mut().query::<&Health>();
            let h = q.iter(ecs.world()).next().unwrap();
            assert!((h.current - 90.0).abs() < f64::EPSILON);
        }

        assert!(ecs.heal_entity(1, 50.0));
        {
            let mut q = ecs.world_mut().query::<&Health>();
            let h = q.iter(ecs.world()).next().unwrap();
            assert!((h.current - 100.0).abs() < f64::EPSILON);
        }

        assert!(ecs.damage_entity(1, 200.0));
        {
            let mut q = ecs.world_mut().query::<&Health>();
            let h = q.iter(ecs.world()).next().unwrap();
            assert!((h.current - 0.0).abs() < f64::EPSILON);
        }
    }

    #[test]
    fn test_inventory_operations() {
        let mut inv = Inventory::new(2);
        assert!(inv.add_item(Item::new(1, "sword", 1)));
        assert!(inv.add_item(Item::new(2, "shield", 1)));
        assert!(!inv.add_item(Item::new(3, "potion", 1)));

        assert!(inv.is_full());

        let removed = inv.remove_item_by_id(1);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().name, "sword");

        let removed_nonexistent = inv.remove_item_by_id(999);
        assert!(removed_nonexistent.is_none());
    }

    #[test]
    fn test_decay_system() {
        let mut ecs = EcsWorld::new();
        ecs.world_mut()
            .spawn((EntityRef::new(1), Health::full(100.0)));

        decay_system(ecs.world_mut(), 5.0, 2.0);

        {
            let mut q = ecs.world_mut().query::<&Health>();
            let h = q.iter(ecs.world()).next().unwrap();
            assert!((h.current - 90.0).abs() < f64::EPSILON);
        }

        decay_system(ecs.world_mut(), 100.0, 10.0);
        {
            let mut q = ecs.world_mut().query::<&Health>();
            let h = q.iter(ecs.world()).next().unwrap();
            assert!((h.current - 0.0).abs() < f64::EPSILON);
        }
    }

    #[test]
    fn test_set_position_and_velocity() {
        let mut ecs = EcsWorld::new();
        ecs.spawn_at(1, 0.0, 0.0, 0.0);

        assert!(ecs.set_position(1, Position::new(10.0, 20.0, 30.0)));
        let pos = ecs.get_position(1).unwrap();
        assert!((pos.x - 10.0).abs() < f64::EPSILON);

        assert!(!ecs.set_position(999, Position::zero()));
    }
}
