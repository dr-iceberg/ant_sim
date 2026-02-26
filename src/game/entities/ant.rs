use ggez::{glam::Vec2, graphics::Rect};

//use std::f32::consts::PI;

use crate::game::rendering::{animation::Animation, assets::AssetMetadata};

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum AntState {
    Idle,
    FoodSearch,
}

#[derive(Debug, Clone)]
pub struct Ant {
    pos: Vec2,
    vel: Vec2,
    state: AntState,
    animation: Animation,
    rect: Rect,
}
#[allow(dead_code)]
impl Ant {
    pub fn new(animation: Animation) -> Self {
        let pos = Vec2 { x: 100., y: 100. };
        Ant {
            pos: pos,
            vel: Vec2 { x: 100., y: 100. },
            state: AntState::FoodSearch,
            animation: Animation::default(),
            rect: Rect {
                x: 0.,
                y: 0.,
                w: metadata.rect().w,
                h: metadata.rect().h,
            },
        }
    }

    pub fn update(&mut self, dt: f32) {
        //self.timer += dt;
        /*
        if self.timer >= 1. {
            let current_angle = self.vel.angle_between(Vec2 { x: 1., y: 0. });
            let angle_offset = PI / 12.;
            let angle =
                rand::random_range((current_angle - angle_offset)..(current_angle + angle_offset));
            self.vel = Vec2::from_angle(angle) * self.vel.length();
            self.timer = 0.;
        }
        */

        match self.state {
            AntState::Idle => {}
            AntState::FoodSearch => {
                self.pos += self.vel * dt;
            }
        }
        self.rect.x = self.pos.x;
        self.rect.y = self.pos.y;
        self.animation.update(dt);
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

    pub fn animation(&self) -> &Animation {
        &self.animation
    }

    pub fn set_animation(&mut self, animation: Animation) {
        self.animation = animation;
    }
}
