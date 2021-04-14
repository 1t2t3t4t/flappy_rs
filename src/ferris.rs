use ggez::{Context, GameResult};
use ggez::event::EventHandler;
use ggez::graphics::{DrawParam, Image, Rect};
use ggez::graphics::mint::Point2;
use ggez::input::keyboard::KeyCode;

use crate::constant::world::{BIRD_HEIGHT, BIRD_WIDTH, JUMP_FORCE};
use crate::constant::world::GRAVITY;
use crate::game_state::{GameComponent, Priority};

#[derive(Debug)]
pub struct Ferris {
    img: Image,
    pub rect: Rect,
    velocity: Point2<f32>,
    is_alive: bool,
}

impl Ferris {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            img: Image::new(ctx, "/cuddlyferris.png").expect("Missing ferris img"),
            rect: Rect::new(100f32, 200f32, BIRD_WIDTH, BIRD_HEIGHT),
            velocity: Point2 { x: 0f32, y: 0f32 },
            is_alive: true,
        }
    }

    pub fn killed(&self) -> bool {
        !self.is_alive
    }

    pub fn kill(&mut self) {
        self.is_alive = false;
    }

    fn hit_floor(&self, frame_h: f32) -> bool {
        self.rect.y + self.rect.h > frame_h
    }
}

impl EventHandler for Ferris {
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
        let draw_param = DrawParam::new()
            .dest([self.rect.x, self.rect.y])
            .offset([0.02, 0.02])
            .scale([0.07, 0.07]);
        ggez::graphics::draw(_ctx, &self.img, draw_param)
    }
}

impl GameComponent for Ferris {
    fn priority(&self) -> Priority {
        Priority::Mid
    }
}
