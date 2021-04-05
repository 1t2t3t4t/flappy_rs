use ggez::{Context, GameResult};
use ggez::event::EventHandler;
use ggez::graphics::{Color, DrawMode, MeshBuilder};
use ggez::graphics::mint::Point2;

use crate::constant::EMPTY_DRAW_PARAM;
use crate::game_state::{GameComponent, Priority};

#[derive(Debug)]
pub struct Shit {
    position: Point2<f32>,
}

impl Default for Shit {
    fn default() -> Self {
        Self {
            position: Point2 { x: 40f32, y: 40f32 },
        }
    }
}

impl EventHandler for Shit {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
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
