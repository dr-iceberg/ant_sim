use ggez::Context;

use crate::game::entities::ant::Ant;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Nest {
    ants: Vec<Ant>,
    food: u32,
}

#[allow(dead_code)]
impl Nest {
    pub fn new(ant_count: i32, starting_food: u32) -> Self {
        Nest {
            ants: vec![Ant::new(); ant_count as usize],
            food: starting_food,
        }
    }

    pub fn update(&mut self, ctx: &mut Context) {
        for ant in self.ants.iter_mut() {
            ant.update(ctx.time.delta().as_secs_f32());
        }
    }

    pub fn ants(&self) -> &[Ant] {
        &self.ants
    }

    pub fn food(&self) -> u32 {
        self.food
    }
}
