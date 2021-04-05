use std::any::{Any, TypeId};
use std::collections::HashMap;

use ggez::event::EventHandler;
use ggez::{Context, ContextBuilder, GameResult};

use crate::background::Background;
use crate::shit::Shit;

mod background;
mod constant;
mod shit;

#[derive(Eq, PartialEq)]
enum Priority {
    Low,
    Mid,
    High,
}

impl Priority {
    fn val(&self) -> u8 {
        match self {
            Priority::Low => 64,
            Priority::Mid => 128,
            Priority::High => 255,
        }
    }
}

trait AsAny {
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

trait GameComponent: EventHandler + AsAny {
    fn priority(&self) -> Priority;
}

#[derive(Default)]
struct GameState {
    components: HashMap<TypeId, Box<dyn GameComponent>>,
}

impl GameState {
    fn add_component(&mut self, new_component: impl GameComponent + 'static) {
        let component_type_id = new_component.type_id();
        if self.components.contains_key(&component_type_id) {
            return;
        }
        self.components
            .insert(component_type_id, Box::new(new_component));
    }

    fn draw_by_priority(&mut self, _ctx: &mut Context, priority: Priority) -> GameResult {
        self.components.values_mut()
            .filter(|x| x.priority() == priority)
            .try_for_each(|x| x.draw(_ctx))?;
        Ok(())
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

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("flappy_shit", "Boss")
        .build()
        .expect("Buildable");

    let mut game_state = GameState::default();
    game_state.add_component(Background);
    game_state.add_component(Shit::default());

    ggez::event::run(&mut ctx, &mut event_loop, &mut game_state).unwrap();
}
