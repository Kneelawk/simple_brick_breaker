use crate::{
    collision::CollisionContext,
    components::{Ball, Collidable},
};
use amethyst::{
    core::{
        math::{Unit, Vector2},
        timing::Time,
        transform::Transform,
    },
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct BallMovementSystem;

impl<'s> System<'s> for BallMovementSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (balls, mut transforms, time): Self::SystemData) {
        for (ball, transform) in (&balls, &mut transforms).join() {
            transform.append_translation_xyz(
                ball.velocity.x * time.delta_seconds(),
                ball.velocity.y * time.delta_seconds(),
                0.0,
            );
        }
    }
}

#[derive(SystemDesc)]
pub struct BallCollisionSystem;

impl<'s> System<'s> for BallCollisionSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Collidable>,
        Read<'s, CollisionContext>,
    );

    fn run(&mut self, (mut balls, collidables, context): Self::SystemData) {
        let world = &context.world;

        for (ball, collidable) in (&mut balls, &collidables).join() {
            if let Some(contacts) = world.collision_objects_in_contact_with(collidable.handle) {
                let mut contacted = false;

                for contact in contacts {
                    if let Some((a, _, _, manifold)) =
                        world.contact_pair(collidable.handle, contact, true)
                    {
                        let normal: Unit<Vector2<f32>> = if a == collidable.handle {
                            manifold.deepest_contact().unwrap().contact.normal
                        } else {
                            -manifold.deepest_contact().unwrap().contact.normal
                        };

                        if ball.velocity.dot(&normal) > 0.0 {
                            ball.velocity -= 2.0 * ball.velocity.dot(&normal) * *normal;
                            contacted = true;
                        }
                    }
                }

                if contacted {
                    println!("Contact!");
                }
            }
        }
    }
}
