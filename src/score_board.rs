use ggez::event::EventHandler;
use ggez::graphics::{Font, Scale, Text, TextFragment};
use ggez::{Context, GameResult};

use crate::game_state::{GameComponent, Priority};

#[derive(Default)]
pub struct ScoreBoard {
    pub score: u32,
}

impl EventHandler for ScoreBoard {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        let (w, _) = ggez::graphics::drawable_size(_ctx);
        let font = Font::default();
        let text = Text::new(TextFragment {
            text: format!("Score: {}", self.score),
            font: Some(font),
            scale: Some(Scale::uniform(35f32)),
            ..Default::default()
        });
        let x_pos = w / 2f32 - (text.width(_ctx) as f32 / 2f32);
        ggez::graphics::draw(_ctx, &text, ([x_pos, 0f32],))
    }
}

impl GameComponent for ScoreBoard {
    fn priority(&self) -> Priority {
        Priority::High
    }
}
