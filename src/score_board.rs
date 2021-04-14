use ggez::event::EventHandler;
use ggez::graphics::{Font, Scale, Text, TextFragment};
use ggez::{Context, GameResult};

use crate::game_state::{GameComponent, Priority};

#[derive(Default)]
pub struct ScoreBoard {
    pub score: u32,
}

fn draw_shadow_text(
    ctx: &mut Context,
    text: &str,
    font: Option<Font>,
    scale: Option<Scale>,
) -> GameResult {
    let (w, _) = ggez::graphics::drawable_size(ctx);
    let text_border = Text::new(TextFragment {
        text: text.to_string(),
        font,
        scale,
        color: Some(ggez::graphics::BLACK),
    });
    let x_pos_border = w / 2f32 - (text_border.width(ctx) as f32 / 2f32);
    ggez::graphics::draw(ctx, &text_border, ([x_pos_border + 2f32, 0f32 + 2f32],))
}

fn draw_text(
    ctx: &mut Context,
    text: &str,
    font: Option<Font>,
    scale: Option<Scale>,
) -> GameResult {
    let (w, _) = ggez::graphics::drawable_size(ctx);
    let text = Text::new(TextFragment {
        text: text.to_string(),
        font,
        scale,
        ..Default::default()
    });
    let x_pos = w / 2f32 - (text.width(ctx) as f32 / 2f32);

    ggez::graphics::draw(ctx, &text, ([x_pos, 0f32],))
}

impl EventHandler for ScoreBoard {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        let text = format!("Score: {}", self.score);
        let font = Some(Font::default());
        let scale = Some(Scale::uniform(40f32));

        draw_shadow_text(_ctx, &text, font, scale)?;
        draw_text(_ctx, &text, font, scale)
    }
}

impl GameComponent for ScoreBoard {
    fn priority(&self) -> Priority {
        Priority::High
    }
}
