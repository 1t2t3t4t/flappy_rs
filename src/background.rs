use ggez::event::EventHandler;
use ggez::graphics::{DrawMode, MeshBuilder, Rect};
use ggez::{Context, GameResult};

use crate::constant::{color, EMPTY_DRAW_PARAM};
use crate::game_state::{GameComponent, Priority};

#[derive(Default, Debug)]
pub struct Background;

impl EventHandler for Background {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let (w, h) = ggez::graphics::drawable_size(ctx);
        let rect = Rect {
            w,
            h,
            ..Default::default()
        };
        let mesh = MeshBuilder::new()
            .rectangle(DrawMode::fill(), rect, color::BLUE)
            .build(ctx)?;
        ggez::graphics::draw(ctx, &mesh, EMPTY_DRAW_PARAM)
    }
}

impl GameComponent for Background {
    fn priority(&self) -> Priority {
        Priority::Low
    }
}
