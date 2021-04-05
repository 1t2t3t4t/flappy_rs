use ggez::event::EventHandler;
use ggez::graphics::{Color, DrawMode, MeshBuilder, Rect};
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};
use rand::{thread_rng, Rng};

use crate::constant::world::{BIRD_SIZE, PILLAR_SPEED, PILLAR_WIDTH};
use crate::constant::EMPTY_DRAW_PARAM;
use crate::game_state::{GameComponent, Priority};

fn draw_pillar_rect(_ctx: &mut Context, rect: Rect) -> GameResult {
    let pillar = MeshBuilder::new()
        .rectangle(DrawMode::fill(), rect, Color::new(0.5, 0.5, 0.5, 1.0))
        .build(_ctx)?;
    ggez::graphics::draw(_ctx, &pillar, EMPTY_DRAW_PARAM)
}

pub struct Pillar {
    upper_rect: Rect,
    lower_rect: Rect,
    velocity: Point2<f32>,
}

impl Pillar {
    pub fn new(x_pos: f32, screen_height: f32) -> Pillar {
        let pillar_hole = BIRD_SIZE * 2f32;
        let rand_hole_pos = thread_rng().gen_range(0f32..=(screen_height - pillar_hole));
        let lower_pillar_y = rand_hole_pos + pillar_hole;

        Pillar {
            upper_rect: Rect::new(x_pos, 0f32, PILLAR_WIDTH, rand_hole_pos),
            lower_rect: Rect::new(
                x_pos,
                lower_pillar_y,
                PILLAR_WIDTH,
                screen_height - lower_pillar_y,
            ),
            velocity: [PILLAR_SPEED, 0f32].into(),
        }
    }

    pub fn is_out_of_screen(&self) -> bool {
        self.upper_rect.x + PILLAR_WIDTH < 0f32
    }
}

impl EventHandler for Pillar {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let delta = ggez::timer::delta(_ctx).as_secs_f32();
        self.upper_rect.x -= self.velocity.x * delta;
        self.lower_rect.x -= self.velocity.x * delta;
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        draw_pillar_rect(_ctx, self.upper_rect)?;
        draw_pillar_rect(_ctx, self.lower_rect)
    }
}

impl GameComponent for Pillar {
    fn priority(&self) -> Priority {
        Priority::Mid
    }
}
