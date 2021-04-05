use ggez::event::EventHandler;
use ggez::graphics::mint::Point2;
use ggez::graphics::{Color, DrawMode, MeshBuilder, Rect};
use ggez::{Context, GameResult};

use crate::constant::world::{BIRD_SIZE, JUMP_FORCE};
use crate::constant::{world::GRAVITY, EMPTY_DRAW_PARAM};
use crate::game_state::{GameComponent, Priority};
use ggez::input::keyboard::KeyCode;

#[derive(Debug)]
pub struct Shit {
    rect: Rect,
    velocity: Point2<f32>,
    is_alive: bool
}

impl Shit {
    pub fn kill(&mut self) {
        self.is_alive = false;
    }

    fn hit_floor(&self, frame_h: f32) -> bool {
        self.rect.y + self.rect.h > frame_h
    }
}

impl Default for Shit {
    fn default() -> Self {
        Self {
            rect: Rect::new(100f32, 200f32, BIRD_SIZE, BIRD_SIZE),
            velocity: Point2 { x: 0f32, y: 0f32 },
            is_alive: true
        }
    }
}

impl EventHandler for Shit {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let delta = ggez::timer::delta(_ctx).as_secs_f32();
        let (_, h) = ggez::graphics::drawable_size(_ctx);
        self.velocity.y += GRAVITY;

        let is_pressed = ggez::input::keyboard::is_key_pressed(_ctx, KeyCode::Space);
        let is_hold = ggez::input::keyboard::is_key_repeated(_ctx);
        if is_pressed && !is_hold && self.is_alive {
            self.velocity.y = JUMP_FORCE;
        }

        self.rect.x += self.velocity.x * delta;

        if self.hit_floor(h) {
            self.kill();
        } else {
            self.rect.y += self.velocity.y * delta;
        }

        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        let cir = MeshBuilder::new()
            .rectangle(DrawMode::fill(), self.rect, Color::new(0.5, 0.5, 0.5, 1.0))
            .build(_ctx)?;
        ggez::graphics::draw(_ctx, &cir, EMPTY_DRAW_PARAM)
    }
}

impl GameComponent for Shit {
    fn priority(&self) -> Priority {
        Priority::Mid
    }
}
