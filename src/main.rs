use std::any::Any;
use std::env;
use std::path;

use ggez::ContextBuilder;

use crate::game_state::GameState;

mod background;
mod constant;
mod ferris;
mod fps_counter;
mod game_state;
mod pillar;
mod pillar_container;
mod save_system;
mod score_board;

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
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
    game_state.set_up(&mut ctx);

    ggez::event::run(&mut ctx, &mut event_loop, &mut game_state).unwrap();
}
