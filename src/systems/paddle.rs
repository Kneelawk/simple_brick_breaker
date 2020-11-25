use crate::{
    components::Paddle,
    game::{ARENA_WIDTH, PADDLE_DISTANCE_VELOCITY_RATIO, PADDLE_MAX_VELOCITY, PADDLE_WIDTH},
};
use amethyst::{
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

#[derive(SystemDesc)]
pub struct PaddleSystem {
    target_x: f32,
}

impl PaddleSystem {
    pub fn new() -> PaddleSystem {
        PaddleSystem { target_x: 0.0 }
    }
}

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, paddles, input, time): Self::SystemData) {
        if let Some((x, _y)) = input.mouse_position() {
            if x < PADDLE_WIDTH / 2.0 {
                self.target_x = PADDLE_WIDTH / 2.0;
            } else if x > ARENA_WIDTH - PADDLE_WIDTH / 2.0 {
                self.target_x = ARENA_WIDTH - PADDLE_WIDTH / 2.0;
            } else {
                self.target_x = x;
            }
        }
        for (_paddle, transform) in (&paddles, &mut transforms).join() {
            let paddle_x = transform.translation().x;
            let diff_x = self.target_x - paddle_x;
            let abs_diff_x = diff_x.abs();
            let vel_x = if abs_diff_x > 0.0 {
                if abs_diff_x > PADDLE_MAX_VELOCITY * PADDLE_DISTANCE_VELOCITY_RATIO {
                    if diff_x > 0.0 {
                        PADDLE_MAX_VELOCITY
                    } else {
                        -PADDLE_MAX_VELOCITY
                    }
                } else {
                    diff_x / PADDLE_DISTANCE_VELOCITY_RATIO
                }
            } else {
                0.0
            };
            transform.set_translation_x(paddle_x + vel_x * time.delta_seconds());
        }
    }
}
