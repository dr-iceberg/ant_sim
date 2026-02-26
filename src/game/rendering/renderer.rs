use ggez::Context;
use ggez::graphics::Rect;
use ggez::{
    glam::Vec2,
    graphics::{Canvas, Color, DrawMode, DrawParam, GraphicsContext, Mesh},
};

use crate::game::entities::nest::Nest;
use crate::game::entities::{ant::Ant, food::Food};
use crate::game::rendering::assets::AssetManager;
use crate::game::world::World;

#[allow(dead_code)]
pub struct Renderer {
    food_mesh: Mesh,
    nest_mesh: Mesh,
}

impl Renderer {
    pub fn new(gfx: &GraphicsContext) -> Self {
        let food_mesh = Mesh::new_circle(
            gfx,
            DrawMode::fill(),
            Vec2 { x: 0., y: 0. },
            50.,
            0.1,
            Color::GREEN,
        )
        .unwrap();

        let nest_mesh = Mesh::new_circle(
            gfx,
            DrawMode::fill(),
            Vec2 { x: 0., y: 0. },
            150.,
            0.1,
            Color::from_rgb(71, 31, 22),
        )
        .unwrap();

        Self {
            food_mesh: food_mesh,
            nest_mesh: nest_mesh,
        }
    }

    pub fn render_world(
        &mut self,
        _ctx: &Context,
        canvas: &mut Canvas,
        world: &World,
        assets: &AssetManager,
    ) {
        self.render_nest(canvas, world.nest());
        for food in world.food_vec() {
            self.render_food(canvas, food);
        }
        for ant in world.nest().ants() {
            self.render_ant(ant, canvas, assets);
        }
    }

    fn render_ant(&mut self, ant: &Ant, canvas: &mut Canvas, assets: &AssetManager) {
        // this is a different angle for rotation
        let angle = ant.vel().angle_between(Vec2 { x: -1., y: 0. });
        let scale = 1.;
        let img = assets.ant_sprite_sheet().img();
        let frame = ant.animation().get_current_frame();
        let rect = Rect {
            x: frame.x / img.width() as f32,
            y: frame.y / img.height() as f32,
            w: frame.w / img.width() as f32,
            h: frame.h / img.height() as f32,
        };

        canvas.draw(
            img,
            DrawParam::default()
                .dest(ant.pos())
                .rotation(angle)
                .offset([0.5, 0.5])
                .scale([scale, scale])
                .src(rect),
        );
    }

    fn render_food(&mut self, canvas: &mut Canvas, food: &Food) {
        canvas.draw(&self.food_mesh, DrawParam::default().dest(food.pos()));
    }

    fn render_nest(&mut self, canvas: &mut Canvas, nest: &Nest) {
        canvas.draw(&self.nest_mesh, DrawParam::default().dest(nest.pos()));
    }
}
