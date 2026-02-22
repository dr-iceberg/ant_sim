use ggez::{
    Context, GameResult, event,
    graphics::{Canvas, Color},
};

use crate::game::{rendering::renderer::Renderer, world::World};

pub struct MainState {
    renderer: Renderer,
    world: World,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState {
            renderer: Renderer::new(&ctx.gfx),
            world: World::new().unwrap(),
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

        self.renderer.render_world(ctx, &mut canvas, &self.world);

        canvas.finish(ctx)
    }
}
