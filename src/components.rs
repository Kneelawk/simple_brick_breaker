use crate::collision::CollisionContext;
use amethyst::{
    core::{math::Vector2, transform::Transform},
    ecs::{Component, DenseVecStorage, NullStorage},
    prelude::{World, WorldExt},
};
use ncollide2d::{
    math::Isometry,
    narrow_phase::ContactEvent,
    pipeline::{CollisionGroups, CollisionObjectSlabHandle, GeometricQueryType},
    shape::{Shape, ShapeHandle},
};
use smallvec::SmallVec;

#[derive(Debug, Default)]
pub struct Paddle;

#[derive(Debug, Clone)]
pub struct Ball {
    pub velocity: Vector2<f32>,
}

#[derive(Clone)]
pub struct Collidable {
    pub handle: CollisionObjectSlabHandle,
}

#[derive(Clone)]
pub struct Contact {
    pub contacts: SmallVec<[ContactEvent<CollisionObjectSlabHandle>; 2]>,
}

impl Component for Paddle {
    type Storage = NullStorage<Paddle>;
}

impl Component for Ball {
    type Storage = DenseVecStorage<Ball>;
}

impl Component for Collidable {
    type Storage = DenseVecStorage<Collidable>;
}

impl Component for Contact {
    type Storage = DenseVecStorage<Contact>;
}

impl Collidable {
    pub fn new_ball<S: Shape<f32>>(world: &World, transform: &Transform, shape: S) -> Collidable {
        let context = &mut *world.write_resource::<CollisionContext>();

        Collidable::new(context, context.ball_groups, transform, shape)
    }

    pub fn new_other<S: Shape<f32>>(world: &World, transform: &Transform, shape: S) -> Collidable {
        let context = &mut *world.write_resource::<CollisionContext>();

        Collidable::new(context, context.other_groups, transform, shape)
    }

    fn new<S: Shape<f32>>(
        context: &mut CollisionContext,
        group: CollisionGroups,
        transform: &Transform,
        shape: S,
    ) -> Collidable {
        let translation = transform.translation();

        let position = Isometry::new(Vector2::new(translation.x, translation.y), 0.0);
        let handle = ShapeHandle::new(shape);

        let (handle, _) = context.world.add(
            position,
            handle,
            group,
            GeometricQueryType::Contacts(0.0, 0.0),
            (),
        );

        Collidable { handle }
    }
}
