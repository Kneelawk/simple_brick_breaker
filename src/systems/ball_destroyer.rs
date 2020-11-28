use crate::components::{Ball, BallDestroyer, Contact, ContactEventData};
use amethyst::{
    core::ecs::{Entities, Join, ReadStorage},
    derive::SystemDesc,
    ecs::{System, SystemData},
};

#[derive(SystemDesc)]
pub struct BallDestroyerSystem;

impl<'s> System<'s> for BallDestroyerSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, BallDestroyer>,
        ReadStorage<'s, Ball>,
        ReadStorage<'s, Contact>,
    );

    fn run(&mut self, (entities, ball_destroyers, balls, contacts): Self::SystemData) {
        for (_ball_destroyer, contact) in (&ball_destroyers, &contacts).join() {
            for &data in contact.contacts.iter() {
                if let ContactEventData::Started { other, .. } = data {
                    if let Some(other) = other {
                        if balls.contains(other) {
                            println!("Removing ball...");

                            entities.delete(other).expect("Error deleting ball");
                        }
                    }
                }
            }
        }
    }
}
