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
        let font = Some(Font::default());
        let scale = Some(Scale::uniform(40f32));
        let text = format!("Score: {}", self.score);

        let text_border = Text::new(TextFragment {
            text: text.to_owned(),
            font,
            scale,
            color: Some(ggez::graphics::BLACK),
        });
        let x_pos_border = w / 2f32 - (text_border.width(_ctx) as f32 / 2f32);
        ggez::graphics::draw(_ctx, &text_border, ([x_pos_border + 2f32, 0f32 + 2f32],))?;

        let text = Text::new(TextFragment {
            text,
            font,
            scale,
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
