use crate::{collision::CollisionContext, components::Collidable};
use amethyst::{
    core::{math::Vector2, transform::Transform},
    derive::SystemDesc,
    ecs::{Join, ReadStorage, System, SystemData, Write},
};
use ncollide2d::math::Isometry;

#[derive(SystemDesc)]
pub struct WorldUpdateSystem;

impl<'s> System<'s> for WorldUpdateSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Collidable>,
        Write<'s, CollisionContext>,
    );

    fn run(&mut self, (transforms, collidables, mut context): Self::SystemData) {
        let world = &mut context.world;

        // Update positions...
        for (transform, collidable) in (&transforms, &collidables).join() {
            let translation = transform.translation();
            let position = Isometry::new(Vector2::new(translation.x, translation.y), 0.0);
            let obj = world.get_mut(collidable.handle).unwrap();
            obj.set_position(position);
        }

        // Update the world...
        world.update();
    }
}
