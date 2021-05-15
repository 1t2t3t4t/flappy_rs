use crate::game_state::{GameComponent, Priority};
use ggez::event::EventHandler;
use ggez::graphics::{Font, Scale, Text, TextFragment};
use ggez::{Context, GameResult};

pub struct FpsCounter;

impl EventHandler for FpsCounter {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        let (_, h) = ggez::graphics::drawable_size(_ctx);
        let font = Some(Font::default());
        let scale = Some(Scale::uniform(15f32));
        let fps = ggez::timer::fps(_ctx);
        let text = format!("fps: {:.2}", fps);

        let text = Text::new(TextFragment {
            text,
            font,
            scale,
            color: Some(ggez::graphics::BLACK),
        });
        let y_pos = h - text.height(_ctx) as f32 - 10 as f32;
        ggez::graphics::draw(_ctx, &text, ([10f32, y_pos],))
    }
}

impl GameComponent for FpsCounter {
    fn priority(&self) -> Priority {
        Priority::High
    }
}
