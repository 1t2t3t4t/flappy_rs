use ggez::event::EventHandler;
use ggez::graphics::{DrawParam, Image};
use ggez::{Context, GameResult};

use crate::constant::world::SKY_SPEED;
use crate::constant::EMPTY_DRAW_PARAM;
use crate::game_state::{GameComponent, Priority};
use ggez::graphics::spritebatch::SpriteBatch;

#[derive(Debug)]
pub struct Background {
    sky_sprite: SpriteBatch,
    sky_img: Image,
    offset: f32,
    play: bool,
}

impl Background {
    pub fn new(ctx: &mut Context) -> Self {
        let sky_img = Image::new(ctx, "/sky.png").expect("Missing sky image");
        Background {
            sky_sprite: SpriteBatch::new(sky_img.clone()),
            sky_img,
            offset: 0f32,
            play: true,
        }
    }

    pub fn stop(&mut self) {
        self.play = false;
    }
}

impl EventHandler for Background {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if !self.play {
            return Ok(());
        }

        let delta_time = ggez::timer::delta(_ctx).as_secs_f32();
        self.offset += SKY_SPEED * delta_time;
        if self.offset >= 1f32 {
            self.offset = 0f32
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.sky_sprite.clear();

        let first_sky = DrawParam::new()
            .dest([0f32, 0f32])
            .offset([self.offset, 0f32]);
        self.sky_sprite.add(first_sky);

        let img_width = self.sky_img.width() as f32;
        let concat_sky = DrawParam::new()
            .dest([img_width, 0f32])
            .offset([self.offset, 0f32]);
        self.sky_sprite.add(concat_sky);

        ggez::graphics::draw(ctx, &self.sky_sprite, EMPTY_DRAW_PARAM)
    }
}

impl GameComponent for Background {
    fn priority(&self) -> Priority {
        Priority::None
    }
}
