use ggez::event::EventHandler;
use ggez::graphics::{Font, PxScale, Text, TextFragment};
use ggez::{Context, GameError, GameResult};

use crate::game_state::{GameComponent, Priority};

#[derive(Default)]
pub struct ScoreBoard {
    pub score: u32,
    pub highest_score: u32
}

fn draw_shadow_text(
    ctx: &mut Context,
    text: &str,
    font: Option<Font>,
    scale: Option<PxScale>,
) -> GameResult {
    let (w, _) = ggez::graphics::drawable_size(ctx);
    let text_border = Text::new(TextFragment {
        text: text.to_string(),
        font,
        scale,
        color: Some(ggez::graphics::Color::BLACK),
    });
    let x_pos_border = w / 2f32 - (text_border.width(ctx) as f32 / 2f32);
    ggez::graphics::draw(ctx, &text_border, ([x_pos_border + 2f32, 0f32 + 2f32],))
}

fn draw_text(
    ctx: &mut Context,
    text: &str,
    font: Option<Font>,
    scale: Option<PxScale>,
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

impl ScoreBoard {
    fn score_text(&self) -> String {
        if self.highest_score > 0 {
            format!("Score: {}. Highscore: {}", self.score, self.highest_score)
        } else {
            format!("Score: {}", self.score)
        }
    }
}

impl EventHandler<GameError> for ScoreBoard {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        let text = self.score_text();
        let font = Some(Font::default());
        let scale = Some(PxScale::from(40f32));

        draw_shadow_text(_ctx, &text, font, scale)?;
        draw_text(_ctx, &text, font, scale)
    }
}

impl GameComponent for ScoreBoard {
    fn priority(&self) -> Priority {
        Priority::High
    }
}
