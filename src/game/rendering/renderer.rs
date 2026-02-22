use ggez::Context;
use ggez::{
    glam::Vec2,
    graphics::{Canvas, Color, DrawParam, GraphicsContext, Mesh},
};

use crate::game::world::World;
use crate::game::entities::{ant::Ant, food::Food};
use crate::game::rendering::assets::AssetManager;

#[allow(dead_code)]
pub struct Renderer {
    assets: AssetManager,
    food_mesh: Mesh,
}

impl Renderer {
    pub fn new(gfx: &GraphicsContext) -> Self {
        let food_mesh = ggez::graphics::Mesh::new_circle(
            gfx,
            ggez::graphics::DrawMode::fill(),
            Vec2 { x: 0., y: 0. },
            50.,
            0.1,
            Color::GREEN,
        )
        .expect("Could not create a mesh");

        Self {
            assets: AssetManager::new(gfx).unwrap(),
            food_mesh: food_mesh,
        }
    }
    
    pub fn render_world(&mut self, _ctx: &Context, canvas: &mut Canvas, world: &World) {
        for ant in world.nest().ants() {
            self.render_ant(ant, canvas);
        }
        for food in world.food_vec() {
            self.render_food(canvas, food);
        }
    }


    fn render_ant(&mut self, ant: &Ant, canvas: &mut Canvas) {
        let angle = ant.vel().angle_between(Vec2 { x: -1., y: 0. });
        let scale = 0.2;
        canvas.draw(
            self.assets.ant_png(),
            DrawParam::default()
                .dest(ant.pos())
                .rotation(angle)
                .offset([0.5, 0.5])
                .scale([scale, scale]),
        );
    }

    fn render_food(&mut self, canvas: &mut Canvas, food: &Food) {
        canvas.draw(&self.food_mesh, DrawParam::default().dest(food.pos()));
    }
}
