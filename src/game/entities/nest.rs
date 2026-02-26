use ggez::{Context, glam::Vec2};

use crate::game::{
    entities::ant::Ant,
    rendering::{animation::AnimationManager, assets::AssetManager},
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Nest {
    ants: Vec<Ant>,
    pos: Vec2,
    food: u32,
}

#[allow(dead_code)]
impl Nest {
    pub fn new(ant_count: i32, starting_food: u32, pos: Vec2, assets: &AssetManager) -> Self {
        let ants: Vec<Ant> = Vec::new();

        for i in 0..ant_count {
            let mut ant = Ant::new();
        }

        Nest {
            ants: vec![Ant::new(); ant_count as usize],
            pos: pos,
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

    pub fn pos(&self) -> Vec2 {
        self.pos
    }
}
