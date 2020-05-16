#![feature(clamp)]

use crate::{
    game::GameState,
    systems::{BallCollisionSystem, BallMovementSystem, PaddleSystem, WorldUpdateSystem},
};
use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod collision;
mod components;
mod game;
mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_bundle(TransformBundle::new())?
        .with(
            PaddleSystem::new(),
            "paddle_system",
            &["input_system", "transform_system"],
        )
        .with(
            BallMovementSystem,
            "ball_movement_system",
            &["transform_system", "paddle_system"],
        )
        .with(
            WorldUpdateSystem::default(),
            "world_update_system",
            &["transform_system", "paddle_system", "ball_movement_system"],
        )
        .with(
            BallCollisionSystem,
            "ball_collision_system",
            &["ball_movement_system", "world_update_system"],
        );

    let mut game = Application::new(assets_dir, GameState, game_data)?;
    game.run();

    Ok(())
}
