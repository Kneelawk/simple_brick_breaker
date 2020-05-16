use crate::{
    collision::CollisionContext,
    components::{Collidable, Contact},
};
use amethyst::{
    core::{
        math::{Isometry2, Vector2},
        transform::Transform,
    },
    derive::SystemDesc,
    ecs::{Entities, Entity, Join, ReadStorage, System, SystemData, Write, WriteStorage},
};
use ncollide2d::{narrow_phase::ContactEvent, pipeline::CollisionObjectSlabHandle};
use smallvec::smallvec;
use std::collections::HashMap;

#[derive(SystemDesc, Default)]
pub struct WorldUpdateSystem {
    entities: HashMap<CollisionObjectSlabHandle, Entity>,
}

impl<'s> System<'s> for WorldUpdateSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Collidable>,
        WriteStorage<'s, Contact>,
        Write<'s, CollisionContext>,
    );

    fn run(
        &mut self,
        (entities, transforms, collidables, mut contacts, mut context): Self::SystemData,
    ) {
        let world = &mut context.world;

        // Update positions...
        for (transform, collidable) in (&transforms, &collidables).join() {
            let translation = transform.translation();
            let position = Isometry2::new(Vector2::new(translation.x, translation.y), 0.0);
            let obj = world.get_mut(collidable.handle).unwrap();
            obj.set_position(position);
        }

        // Update the world...
        world.update();

        // Update contact components...
        for (entity, collidable) in (&*entities, &collidables).join() {
            self.entities.insert(collidable.handle, entity);
            contacts.remove(entity);
        }

        fn insert_event(
            entities: &HashMap<CollisionObjectSlabHandle, Entity>,
            contacts: &mut WriteStorage<'_, Contact>,
            handle: CollisionObjectSlabHandle,
            event: ContactEvent<CollisionObjectSlabHandle>,
        ) {
            if let Some(&entity) = entities.get(&handle) {
                if let Some(contact) = contacts.get_mut(entity) {
                    contact.contacts.push(event);
                } else {
                    contacts
                        .insert(
                            entity,
                            Contact {
                                contacts: smallvec![event],
                            },
                        )
                        .unwrap();
                }
            }
        }

        for &event in world.contact_events() {
            match event {
                ContactEvent::Started(a, b) => {
                    insert_event(&self.entities, &mut contacts, a, event);
                    insert_event(&self.entities, &mut contacts, b, event);
                }
                ContactEvent::Stopped(a, b) => {
                    insert_event(&self.entities, &mut contacts, a, event);
                    insert_event(&self.entities, &mut contacts, b, event);
                }
            }
        }

        self.entities.clear();
    }
}
