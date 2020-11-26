use crate::{
    collision::initialize_collision_context,
    components::{Ball, Collidable, Paddle},
};
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{math::Vector2, transform::Transform},
    prelude::{Builder, World, WorldExt},
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    GameData, SimpleState, StateData,
};
use ncollide2d::shape::Cuboid;
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

pub const ARENA_WIDTH: f32 = 1280.0;
pub const ARENA_HEIGHT: f32 = 720.0;
pub const PADDLE_WIDTH: f32 = 128.0;
pub const PADDLE_HEIGHT: f32 = 32.0;
pub const PADDLE_MAX_VELOCITY: f32 = 720.0;
pub const PADDLE_DISTANCE_VELOCITY_RATIO: f32 = 1.0 / 6.0;
pub const BALL_WIDTH: f32 = 16.0;
pub const BALL_HEIGHT: f32 = 16.0;
pub const BALL_INITIAL_SPEED: f32 = 60.0;
pub const BALL_MAX_SPEED: f32 = 540.0;
pub const BALL_MAX_ROTATION: f32 = PI / 12.0;
pub const BALL_MIN_SCALE: f32 = 1.0;
pub const BALL_MAX_SCALE: f32 = 1.1;

pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<Collidable>();
        world.register::<Ball>();

        let sprite_sheet_handle = load_sprite_sheet(world);

        initialize_collision_context(world);

        initialize_camera(world);
        initialize_paddle(world, sprite_sheet_handle.clone());

        let mut rand = thread_rng();
        let dir = rand.gen_range(0.0, 2.0 * PI);
        initialize_ball(
            world,
            sprite_sheet_handle,
            Vector2::new(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0),
            Vector2::new(
                BALL_INITIAL_SPEED * dir.cos(),
                BALL_INITIAL_SPEED * dir.sin(),
            ),
        );
    }
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/game_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/game_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_storage,
    )
}

fn initialize_paddle(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH / 2.0, PADDLE_HEIGHT / 2.0, 0.0);

    let render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    let shape = Cuboid::new(Vector2::new(PADDLE_WIDTH / 2.0, PADDLE_HEIGHT / 2.0));
    let collidable = Collidable::new_other(world, &transform, shape);

    world
        .create_entity()
        .with(Paddle)
        .with(collidable)
        .with(transform)
        .with(render)
        .build();
}

fn initialize_ball(
    world: &mut World,
    sprite_sheet_handle: Handle<SpriteSheet>,
    position: Vector2<f32>,
    velocity: Vector2<f32>,
) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(position.x, position.y, 0.0);

    let ball = Ball { velocity };

    let render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 1,
    };

    let shape = Cuboid::new(Vector2::new(BALL_WIDTH / 2.0, BALL_HEIGHT / 2.0));
    let collidable = Collidable::new_ball(world, &transform, shape);

    world
        .create_entity()
        .with(ball)
        .with(collidable)
        .with(transform)
        .with(render)
        .build();
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}
