use ggez::{
    glam::Vec2,
    graphics::{Canvas, DrawParam}
};

use crate::game::{rendering::assets::AssetManager};
use std::f32::consts::PI;

#[derive(Debug, Clone)]
enum AntState {
    Idle,
    FoodSearch,
}

#[derive(Debug, Clone)]
pub struct Ant {
    pos: Vec2,
    vel: Vec2,
    state: AntState,
    timer: f32
}

impl Ant {
    pub fn new() -> Self {
        Ant {
            pos: Vec2 { x: 100., y: 100. },
            vel: Vec2 { x: 1., y: 1. },
            state: AntState::FoodSearch,
            timer: 0.
        }
    }

    pub fn update(&mut self, dt: f32) {
        //self.timer += dt;
        if self.timer >= 1.
        {
            let current_angle = self.vel.angle_between(Vec2 { x: 1., y: 0. });
            let angle_offset = PI / 12.;
            let angle = rand::random_range((current_angle - angle_offset)..(current_angle + angle_offset));
            self.vel = Vec2::from_angle(angle) * self.vel.length();
            self.timer = 0.;
        }
        self.pos += self.vel;
    }

    pub fn render(&self, canvas: &mut Canvas, asset_manager: &AssetManager) {
        let angle = self.vel.angle_between(Vec2 { x: -1., y: 0. });
        let scale = 0.2;
        canvas.draw(
            asset_manager.ant_png(),
            DrawParam::default()
                .dest(self.pos)
                .rotation(angle)
                .offset([0.5, 0.5])
                .scale([scale, scale])
        );
        
    }
}
