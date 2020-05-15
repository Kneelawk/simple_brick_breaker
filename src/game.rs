use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, NullStorage},
    prelude::{Builder, World, WorldExt},
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    GameData, SimpleState, StateData,
};

pub const ARENA_WIDTH: f32 = 1280.0;
pub const ARENA_HEIGHT: f32 = 720.0;
pub const PADDLE_WIDTH: f32 = 128.0;
pub const PADDLE_HEIGHT: f32 = 32.0;
pub const PADDLE_MAX_VELOCITY: f32 = 720.0;
pub const PADDLE_DISTANCE_VELOCITY_RATIO: f32 = 1.0 / 6.0;
pub const BALL_WIDTH: f32 = 16.0;
pub const BALL_HEIGHT: f32 = 16.0;

pub struct GameState;

#[derive(Debug, Copy, Clone, Default)]
pub struct Paddle;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(world);

        initialize_camera(world);
        initialize_paddle(world, sprite_sheet_handle);
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

    world
        .create_entity()
        .with(Paddle)
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

impl Component for Paddle {
    type Storage = NullStorage<Paddle>;
}
