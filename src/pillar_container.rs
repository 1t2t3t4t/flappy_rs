use ggez::event::EventHandler;
use ggez::{Context, GameError, GameResult};

use crate::constant::world::{BIRD_HEIGHT, PILLAR_SPEED};
use crate::game_state::{GameComponent, Priority};
use crate::pillar::Pillar;

#[derive(Default)]
pub struct PillarContainer {
    pillars: Vec<Pillar>,
}

impl PillarContainer {
    pub fn pillars(&self) -> &Vec<Pillar> {
        &self.pillars
    }

    pub fn pillars_mut(&mut self) -> &mut Vec<Pillar> {
        &mut self.pillars
    }

    pub fn gen_pillar(&mut self, ctx: &mut Context) {
        let (w, h) = ggez::graphics::drawable_size(ctx);
        self.pillars.push(Pillar::new(w, h, PILLAR_SPEED));
    }

    pub fn stop_all(&mut self) {
        self.pillars.iter_mut().for_each(|x| x.stop());
    }

    fn update_pillars(&mut self, ctx: &mut Context) {
        let (w, _) = ggez::graphics::drawable_size(ctx);
        if self.pillars().len() < 10 {
            if let Some(latest) = self.pillars().last() {
                let least_pillar_distance = BIRD_HEIGHT * 6f32;
                if w - latest.pos_x() >= least_pillar_distance {
                    self.gen_pillar(ctx);
                }
            } else {
                self.gen_pillar(ctx);
            }
        }
        self.clean_pillars();
    }

    fn clean_pillars(&mut self) {
        self.pillars = self
            .pillars
            .iter()
            .filter(|x| !x.is_out_of_screen())
            .map(|x| x.to_owned())
            .collect();
    }
}

impl EventHandler<GameError> for PillarContainer {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.update_pillars(_ctx);
        self.pillars.iter_mut().try_for_each(|x| x.update(_ctx))
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        self.pillars.iter_mut().try_for_each(|x| x.draw(_ctx))
    }
}

impl GameComponent for PillarContainer {
    fn priority(&self) -> Priority {
        Priority::Low
    }
}
