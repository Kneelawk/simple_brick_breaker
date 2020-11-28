use crate::{
    collision::CollisionContext,
    components::{Ball, Collidable, Contact, ContactEventData},
    game::{BALL_MAX_ROTATION, BALL_MAX_SCALE, BALL_MAX_SPEED, BALL_MIN_SCALE},
};
#[allow(unused_imports)]
use amethyst::core::alga::linear::Transformation;
use amethyst::{
    core::{
        math::{Matrix3, Unit, Vector2},
        timing::Time,
        transform::Transform,
    },
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};
use rand::{thread_rng, Rng};

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
        ReadStorage<'s, Contact>,
        Read<'s, CollisionContext>,
    );

    fn run(&mut self, (mut balls, collidables, contacts, context): Self::SystemData) {
        let world = &context.world;
        let mut rand = thread_rng();

        for (ball, collidable, contact) in (&mut balls, &collidables, &contacts).join() {
            let mut contacted = false;

            for &data in contact.contacts.iter() {
                if let ContactEventData::Started {
                    you_handle,
                    other_handle,
                    ..
                } = data
                {
                    if let Some((relative, _, _, manifold)) =
                        world.contact_pair(you_handle, other_handle, true)
                    {
                        let normal: Unit<Vector2<f32>> = if relative == collidable.handle {
                            manifold.deepest_contact().unwrap().contact.normal.clone()
                        } else {
                            -manifold.deepest_contact().unwrap().contact.normal.clone()
                        };

                        if ball.velocity.dot(&normal) > 0.0 {
                            ball.velocity -= 2.0 * ball.velocity.dot(&normal) * *normal;

                            // randomly adjust ball direction
                            let diff = (ball.velocity.dot(&normal) / ball.velocity.norm())
                                .asin()
                                .abs()
                                .min(BALL_MAX_ROTATION);
                            let rot = rand.gen_range(-diff, diff);
                            let rot_mat: Matrix3<f32> = Matrix3::new_rotation(rot);
                            ball.velocity = rot_mat.transform_vector(&ball.velocity);

                            contacted = true;
                        }
                    }
                }
            }

            if contacted {
                println!("Contact!");

                // randomly increase ball speed
                if ball.velocity.norm() < BALL_MAX_SPEED {
                    ball.velocity =
                        rand.gen_range(BALL_MIN_SCALE, BALL_MAX_SCALE) * ball.velocity.clone();
                }
            }
        }
    }
}
