use crate::game::rendering::assets::AssetManager;
use ggez::{
    Context, GameResult, event,
    graphics::{Canvas, Color},
};

use crate::game::{
    rendering::{animation::AnimationManager, renderer::Renderer},
    world::World,
};

pub struct MainState {
    assets: AssetManager,
    renderer: Renderer,
    animations: AnimationManager,
    world: World,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        let assets = AssetManager::new(&ctx.gfx).unwrap();
        let animations = AnimationManager::new(&assets);
        let world = World::new(&assets).unwrap();
        Ok(MainState {
            assets: assets,
            renderer: Renderer::new(&ctx.gfx),
            animations: animations,
            world: world,
        })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.world.update(ctx, &self.animations);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::from([0.1, 0.2, 0.3, 1.0]));

        self.renderer
            .render_world(ctx, &mut canvas, &self.world, &self.assets);

        canvas.finish(ctx)
    }
}
