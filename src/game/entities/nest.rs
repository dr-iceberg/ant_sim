
use ggez::Context;
use ggez::graphics::Canvas;

use crate::game::entities::ant::Ant;
use crate::game::rendering::assets::AssetManager;

#[derive(Debug)]
pub struct Nest {
    ants: Vec<Ant>,
    food: i32,
}

impl Nest {
    pub fn new(ant_count: i32, starting_food: i32) -> Self {
        Nest {
            ants: vec![Ant::new(); ant_count as usize],
            food: starting_food,
        }
    }
    
    pub fn update(&mut self, ctx: &mut Context)
    {
        
        for ant in self.ants.iter_mut()
        {
            ant.update(ctx.time.delta().as_secs_f32());
        }
    }
    
    pub fn render(&self, canvas: &mut Canvas, asset_manager: &AssetManager)
    {
        
        for ant in self.ants.iter()
        {
            ant.render(canvas, asset_manager);
        }
    }
}
