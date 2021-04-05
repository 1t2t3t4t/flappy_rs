use std::any::Any;

use ggez::{ContextBuilder, GameResult};

use crate::background::Background;
use crate::game_state::{GameComponentContainer, GameState};
use crate::pillar::Pillar;
use crate::shit::Shit;

mod background;
mod constant;
mod game_state;
mod pillar;
mod shit;

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("flappy_shit", "Boss")
        .build()
        .expect("Buildable");

    let (w, h) = ggez::graphics::drawable_size(&ctx);

    let mut game_state = GameState::default();
    game_state.add_component(Background);
    game_state.add_component(Shit::default());
    game_state.add_component(Pillar::new(w, h));

    ggez::event::run(&mut ctx, &mut event_loop, &mut game_state).unwrap();
}
