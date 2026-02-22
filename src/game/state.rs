use ggez::{
    Context, GameResult, event,
    graphics::{Canvas, Color},
};

use crate::game::{rendering::assets::AssetManager, world::World};

pub struct MainState {
    asset_manager: AssetManager,
    world: World,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState {
            asset_manager: AssetManager::new(&ctx.gfx).unwrap(),
            world: World::new(ctx).unwrap(),
        })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.world.update(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::from([0.1, 0.2, 0.3, 1.0]));
        self.world.render(&mut canvas, &self.asset_manager);
        canvas.finish(ctx)
    }
}
