use ggez::{Context, GameResult};
use ggez::event::EventHandler;
use ggez::graphics::{Color, DrawMode, MeshBuilder, Rect};
use ggez::graphics::mint::Point2;

use crate::constant::{EMPTY_DRAW_PARAM, world::GRAVITY};
use crate::game_state::{GameComponent, Priority};
use ggez::input::keyboard::KeyCode;
use crate::constant::world::{JUMP_FORCE, BIRD_SIZE};

#[derive(Debug)]
pub struct Shit {
    rect: Rect,
    velocity: Point2<f32>
}

impl Default for Shit {
    fn default() -> Self {
        Self {
            rect: Rect::new(100f32, 200f32, BIRD_SIZE, BIRD_SIZE),
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

        self.rect.x += self.velocity.x * delta;
        self.rect.y += self.velocity.y * delta;
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        let cir = MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                self.rect,
                Color::new(0.5, 0.5, 0.5, 1.0)
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
