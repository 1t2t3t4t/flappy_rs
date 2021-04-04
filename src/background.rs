use ggez::event::EventHandler;
use ggez::{Context, GameResult};
use ggez::graphics::{Rect, MeshBuilder, DrawMode, Color};
use crate::{EMPTY_DRAW_PARAM, GameComponent, Priority};
use std::any::Any;

#[derive(Default, Debug)]
pub struct Background;

impl EventHandler for Background {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let (w, h) = ggez::graphics::drawable_size(ctx);
        let rect = Rect {
            w, h,
            ..Default::default()
        };
        let mesh = MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                rect,
                Color::new(0.529, 0.808, 0.922, 1.0)
            ).build(ctx)?;
        ggez::graphics::draw(ctx, &mesh, EMPTY_DRAW_PARAM)
    }
}

impl GameComponent for Background {
    fn priority(&self) -> Priority {
        Priority::None
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}