use ggez::{
    glam::Vec2,
};

use std::f32::consts::PI;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum AntState {
    Idle,
    FoodSearch,
}

#[derive(Debug, Clone)]
pub struct Ant {
    pos: Vec2,
    vel: Vec2,
    state: AntState,
    timer: f32,
}
#[allow(dead_code)]
impl Ant {
    pub fn new() -> Self {
        Ant {
            pos: Vec2 { x: 100., y: 100. },
            vel: Vec2 { x: 100., y: 100. },
            state: AntState::FoodSearch,
            timer: 0.,
        }
    }

    pub fn update(&mut self, dt: f32) {
        //self.timer += dt;
        if self.timer >= 1. {
            let current_angle = self.vel.angle_between(Vec2 { x: 1., y: 0. });
            let angle_offset = PI / 12.;
            let angle =
                rand::random_range((current_angle - angle_offset)..(current_angle + angle_offset));
            self.vel = Vec2::from_angle(angle) * self.vel.length();
            self.timer = 0.;
        }
        self.pos += self.vel * dt;
    }


    pub fn pos(&self) -> Vec2 {
        self.pos
    }

    pub fn vel(&self) -> Vec2 {
        self.vel
    }
    
    pub fn state(&self) -> &AntState {
        &self.state
    }
}
