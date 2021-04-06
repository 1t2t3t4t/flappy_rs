use std::any::{Any, TypeId};
use std::collections::HashMap;

use ggez::event::EventHandler;
use ggez::{Context, GameResult};

use crate::constant::world::PILLAR_WIDTH;
use crate::pillar_container::PillarContainer;
use crate::score_board::ScoreBoard;
use crate::shit::Shit;
use crate::AsAny;

#[derive(Eq, PartialEq)]
pub enum Priority {
    Low,
    Mid,
    High,
}

pub trait GameComponent: EventHandler + AsAny {
    fn priority(&self) -> Priority;
}

pub trait GameComponentContainer {
    fn add_component(&mut self, new_component: impl GameComponent + 'static);

    fn find_component<T: 'static>(&self) -> Option<&T>;
    fn find_component_mut<T: 'static>(&mut self) -> Option<&mut T>;
}

#[derive(Default)]
pub struct GameState {
    components: HashMap<TypeId, Box<dyn GameComponent>>,
    score: u32,
}

impl GameState {
    fn draw_by_priority(&mut self, _ctx: &mut Context, priority: Priority) -> GameResult {
        self.components
            .values_mut()
            .filter(|x| x.priority() == priority)
            .try_for_each(|x| x.draw(_ctx))
    }

    fn check_shit(&mut self) {
        let shit_rect = self.find_component::<Shit>().expect("Shit").rect;
        let container = self
            .find_component_mut::<PillarContainer>()
            .expect("Container");
        let mut should_die = false;

        for pillar in container.pillars_mut() {
            if pillar.collide(shit_rect) {
                should_die = true;
                break;
            } else if shit_rect.x > pillar.pos_x() + PILLAR_WIDTH && !pillar.passed {
                pillar.passed = true;
                self.score += 1;
                break;
            }
        }

        if should_die {
            let shit = self.find_component_mut::<Shit>().expect("Shit");
            shit.kill();
        }
    }

    fn update_score_board(&mut self) {
        let score = self.score;
        let score_board = self.find_component_mut::<ScoreBoard>().expect("Scoreboard");
        score_board.score = score;
    }

    fn check_game_status(&mut self) {
        let killed = self.find_component::<Shit>().expect("Shit").killed();
        let container = self
            .find_component_mut::<PillarContainer>()
            .expect("Container");

        if killed {
            container.stop_all();
        }
    }
}

impl GameComponentContainer for GameState {
    fn add_component(&mut self, new_component: impl GameComponent + 'static) {
        let component_type_id = new_component.type_id();
        if self.components.contains_key(&component_type_id) {
            return;
        }
        self.components
            .insert(component_type_id, Box::new(new_component));
    }

    fn find_component<T: 'static>(&self) -> Option<&T> {
        self.components
            .get(&TypeId::of::<T>())
            .and_then(|x| (**x).as_any().downcast_ref::<T>())
    }

    fn find_component_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.components
            .get_mut(&TypeId::of::<T>())
            .and_then(|x| (**x).as_any_mut().downcast_mut::<T>())
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for component in self.components.values_mut() {
            component.update(_ctx)?
        }

        self.check_shit();
        self.check_game_status();
        self.update_score_board();

        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        ggez::graphics::clear(_ctx, ggez::graphics::BLACK);

        self.draw_by_priority(_ctx, Priority::Low)?;
        self.draw_by_priority(_ctx, Priority::Mid)?;
        self.draw_by_priority(_ctx, Priority::High)?;

        ggez::graphics::present(_ctx)
    }
}
