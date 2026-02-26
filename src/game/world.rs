use crate::game::{
    entities::{food::Food, nest::Nest},
    rendering::{animation::AnimationManager, assets::AssetManager},
};
use ggez::{Context, GameResult, glam::Vec2};

pub struct World {
    nest: Nest,
    food_vec: Vec<Food>,
}

impl World {
    pub fn new(assets: &AssetManager) -> GameResult<World> {
        let ant_count = 1;
        let starting_nest_food = 100;
        let food_vec = vec![Food::new(Vec2 { x: 500., y: 100. })];
        Ok(World {
            nest: Nest::new(ant_count, starting_nest_food, Vec2 { x: 0., y: 0. }, assets),
            food_vec: food_vec,
        })
    }

    pub fn update(&mut self, ctx: &mut Context, _animations: &AnimationManager) {
        self.nest.update(ctx);
    }

    pub fn nest(&self) -> &Nest {
        &self.nest
    }

    pub fn food_vec(&self) -> &[Food] {
        &self.food_vec
    }
}
