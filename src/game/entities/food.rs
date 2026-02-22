use ggez::{glam::Vec2, graphics::Canvas};

use crate::game::rendering::assets::AssetManager;

#[derive(Debug)]
pub struct Food {
    pos: Vec2,
    energy: i32,
}

impl Food {
    pub fn new() -> Self {
        Food {
            pos: Vec2 { x: 100., y: 100. },
            energy: 50,
        }
    }

    pub fn render(&self, canvas: &Canvas, asset_manager: &AssetManager) {}
}
