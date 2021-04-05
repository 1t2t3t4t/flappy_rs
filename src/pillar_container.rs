use std::rc::{Weak, Rc};
use crate::game_state::{GameState, GameComponent, Priority};
use crate::pillar::Pillar;
use ggez::event::EventHandler;
use ggez::{Context, GameResult};

#[derive(Default)]
pub struct PillarContainer {
    pillars: Vec<Pillar>
}

impl PillarContainer {
    pub fn pillars(&self) -> &Vec<Pillar> {
        &self.pillars
    }

    pub fn gen_pillar(&mut self, ctx: &mut Context) {
        let (w, h) = ggez::graphics::drawable_size(ctx);
        self.pillars.push(Pillar::new(w, h));
    }
}

impl EventHandler for PillarContainer {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pillars
            .iter_mut()
            .try_for_each(|x| x.update(_ctx))
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        self.pillars
            .iter_mut()
            .try_for_each(|x| x.draw(_ctx))
    }
}

impl GameComponent for PillarContainer {
    fn priority(&self) -> Priority {
        Priority::Mid
    }
}
