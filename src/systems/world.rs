use crate::{
    collision::CollisionContext,
    components::{Collidable, Contact, ContactEventData},
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

        for &event in world.contact_events() {
            match event {
                ContactEvent::Started(a, b) => {
                    insert_event(&self.entities, &mut contacts, a, b, true);
                    insert_event(&self.entities, &mut contacts, b, a, true);
                }
                ContactEvent::Stopped(a, b) => {
                    insert_event(&self.entities, &mut contacts, a, b, false);
                    insert_event(&self.entities, &mut contacts, b, a, false);
                }
            }
        }

        self.entities.clear();
    }
}

fn insert_event(
    entities: &HashMap<CollisionObjectSlabHandle, Entity>,
    contacts: &mut WriteStorage<'_, Contact>,
    handle: CollisionObjectSlabHandle,
    other: CollisionObjectSlabHandle,
    started: bool,
) {
    if let Some(&entity) = entities.get(&handle) {
        if let Some(contact) = contacts.get_mut(entity) {
            contact
                .contacts
                .push(create_contact(entities, handle, other, started, entity));
        } else {
            contacts
                .insert(
                    entity,
                    Contact {
                        contacts: smallvec![create_contact(
                            entities, handle, other, started, entity
                        )],
                    },
                )
                .unwrap();
        }
    }
}

fn create_contact(
    entities: &HashMap<CollisionObjectSlabHandle, Entity>,
    handle: CollisionObjectSlabHandle,
    other: CollisionObjectSlabHandle,
    started: bool,
    entity: Entity,
) -> ContactEventData {
    if started {
        ContactEventData::Started {
            you_handle: handle,
            other_handle: other,
            you: entity,
            other: entities.get(&handle).map(ToOwned::to_owned),
        }
    } else {
        ContactEventData::Stopped {
            you_handle: handle,
            other_handle: other,
            you: entity,
            other: entities.get(&handle).map(ToOwned::to_owned),
        }
    }
}
