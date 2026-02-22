use ggez::{
    Context,
    glam::*,
    graphics::{self, Color},
};

/// An utility struct for a simple circle
pub struct Circle {
    pos: Vec2,
    radius: f32,
    mesh: graphics::Mesh,
}

impl Circle {
    pub fn new(ctx: &Context, pos: Vec2, radius: f32, color: Color) -> Self {
        Self {
            pos,
            radius,
            mesh: graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Vec2 { x: 0., y: 0. },
                radius,
                0.1,
                color,
            )
            .expect("Could not create a mesh"),
        }
    }

    pub fn mesh(&self) -> &graphics::Mesh {
        &self.mesh
    }

    pub fn pos(&self) -> Vec2 {
        self.pos
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn pos_mut(&mut self) -> &mut Vec2 {
        &mut self.pos
    }
}
