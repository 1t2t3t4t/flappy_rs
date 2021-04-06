use std::any::Any;

use ggez::ContextBuilder;

use crate::background::Background;
use crate::game_state::{GameComponentContainer, GameState};
use crate::pillar_container::PillarContainer;
use crate::score_board::ScoreBoard;
use crate::ferris::Ferris;
use std::env;
use std::path;

mod background;
mod constant;
mod game_state;
mod pillar;
mod pillar_container;
mod score_board;
mod ferris;

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
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (mut ctx, mut event_loop) = ContextBuilder::new("flappy_shit", "Boss")
        .add_resource_path(resource_dir)
        .build()
        .expect("Buildable");

    let mut game_state = GameState::default();
    game_state.add_component(Background);
    game_state.add_component(Ferris::new(&mut ctx));
    game_state.add_component(PillarContainer::default());
    game_state.add_component(ScoreBoard::default());

    ggez::event::run(&mut ctx, &mut event_loop, &mut game_state).unwrap();
}
