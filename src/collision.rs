use crate::game::{ARENA_HEIGHT, ARENA_WIDTH};
use amethyst::{
    core::math::{Isometry2, Vector2},
    prelude::World,
};
use ncollide2d::{
    pipeline::{CollisionGroups, GeometricQueryType},
    shape::{Plane, ShapeHandle},
    world::CollisionWorld,
};

pub struct CollisionContext {
    pub ball_groups: CollisionGroups,
    pub other_groups: CollisionGroups,
    pub world: CollisionWorld<f32, ()>,
}

impl Default for CollisionContext {
    fn default() -> CollisionContext {
        let mut ball_groups = CollisionGroups::new();
        ball_groups.set_membership(&[1]);

        let mut other_groups = CollisionGroups::new();
        other_groups.set_membership(&[2]);
        other_groups.set_whitelist(&[1]);

        let world = CollisionWorld::<f32, ()>::new(1.0);

        CollisionContext {
            ball_groups,
            other_groups,
            world,
        }
    }
}

pub fn initialize_collision_context(world: &mut World) {
    let plane_left = ShapeHandle::new(Plane::new(Vector2::x_axis()));
    let plane_top = ShapeHandle::new(Plane::new(-Vector2::y_axis()));
    let plane_right = ShapeHandle::new(Plane::new(-Vector2::x_axis()));

    let plane_left_pos = Isometry2::new(Vector2::new(0.0, ARENA_HEIGHT / 2.0), 0.0);
    let plane_top_pos = Isometry2::new(Vector2::new(ARENA_WIDTH / 2.0, ARENA_HEIGHT), 0.0);
    let plane_right_pos = Isometry2::new(Vector2::new(ARENA_WIDTH, ARENA_HEIGHT / 2.0), 0.0);

    let mut collision_context = CollisionContext::default();

    let other_groups = collision_context.other_groups;
    let contact_query = GeometricQueryType::Contacts(0.0, 0.0);

    collision_context
        .world
        .add(plane_left_pos, plane_left, other_groups, contact_query, ());
    collision_context
        .world
        .add(plane_top_pos, plane_top, other_groups, contact_query, ());
    collision_context.world.add(
        plane_right_pos,
        plane_right,
        other_groups,
        contact_query,
        (),
    );

    world.insert(collision_context);
}
