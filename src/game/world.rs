use crate::game::entities::nest::Nest;
use crate::game::rendering::assets::AssetManager;
use ggez::{Context, GameResult, graphics::Canvas};

pub struct World {
    nest: Nest,
}

impl World {
    pub fn new(ctx: &Context) -> GameResult<World> {
        Ok(World {
            nest: Nest::new(1, 100),
        })
    }

    pub fn update(&mut self, ctx: &mut Context) {
        self.nest.update(ctx);
    }

    pub fn render(&mut self, canvas: &mut Canvas, asset_manager: &AssetManager) {
        self.nest.render(canvas, asset_manager);
    }
}
