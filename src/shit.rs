use ggez::{Context, GameResult};
use ggez::event::EventHandler;
use ggez::graphics::{Color, DrawMode, MeshBuilder};
use ggez::graphics::mint::Point2;

use crate::constant::{EMPTY_DRAW_PARAM, world::GRAVITY};
use crate::game_state::{GameComponent, Priority};
use ggez::input::keyboard::KeyCode;

const JUMP_FORCE: f32 = -300f32;

#[derive(Debug)]
pub struct Shit {
    position: Point2<f32>,
    velocity: Point2<f32>
}

impl Default for Shit {
    fn default() -> Self {
        Self {
            position: Point2 { x: 150f32, y: 200f32 },
            velocity: Point2 { x: 0f32, y: 0f32 }
        }
    }
}

impl EventHandler for Shit {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let delta = ggez::timer::delta(_ctx).as_secs_f32();
        self.velocity.y += GRAVITY;

        let is_pressed = ggez::input::keyboard::is_key_pressed(_ctx, KeyCode::Space);
        let is_hold = ggez::input::keyboard::is_key_repeated(_ctx);
        if is_pressed && !is_hold {
            self.velocity.y = JUMP_FORCE;
        }

        self.position.x += self.velocity.x * delta;
        self.position.y += self.velocity.y * delta;
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        let cir = MeshBuilder::new()
            .circle(
                DrawMode::fill(),
                self.position,
                40f32,
                1f32,
                Color::new(0.5, 0.5, 0.5, 1.0),
            )
            .build(_ctx)?;
        ggez::graphics::draw(_ctx, &cir, EMPTY_DRAW_PARAM)
    }
}

impl GameComponent for Shit {
    fn priority(&self) -> Priority {
        Priority::Mid
    }
}
