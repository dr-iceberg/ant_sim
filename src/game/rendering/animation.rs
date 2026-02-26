use std::collections::HashMap;

use ggez::graphics::Rect;

use crate::game::{entities::ant::AntState, rendering::assets::AssetManager};

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct Animation {
    sprite_frames: Vec<Rect>,
    current_frame: usize,
    /// as seconds
    frame_time: f32,
    timer: f32,
}

pub struct AnimationManager {
    ant_animations: HashMap<AntState, Animation>,
}

impl Animation {
    pub fn update(&mut self, dt: f32) {
        self.timer += dt;
        if self.timer >= self.frame_time {
            self.current_frame = (self.current_frame + 1) % self.sprite_frames.len();
        }
    }
    pub fn get_current_frame(&self) -> Rect {
        self.sprite_frames[self.current_frame]
    }
}

impl AnimationManager {
    pub fn new(assets: &AssetManager) -> Self {
        let mut ant_animations = HashMap::new();
        ant_animations.insert(AntState::Idle, Animation::default());
        ant_animations.insert(AntState::FoodSearch, Self::ant_walk(assets));

        AnimationManager {
            ant_animations: ant_animations,
        }
    }

    pub fn ant_walk(assets: &AssetManager) -> Animation {
        let sprite_rows = 8;
        let sprite_columns = 8;
        let sprite_count = 62;
        let frame_width = assets.ant_sprite_sheet().img().width() / sprite_columns;
        let frame_height = assets.ant_sprite_sheet().img().height() / sprite_rows;
        let mut frames: Vec<Rect> = Vec::new();
        for i in 0..sprite_count {
            let col = i % sprite_columns;
            let row = ((i / sprite_columns) as f32).floor() as u32;
            let x = (frame_width * col) as f32;
            let y = (frame_height * row) as f32;
            let w = frame_width as f32;
            let h = frame_height as f32;
            let rect = Rect::new(x, y, w, h);

            frames.push(rect);
        }

        Animation {
            sprite_frames: frames,
            current_frame: 0,
            frame_time: 0.2,
            timer: 0.,
        }
    }

    pub fn ant_animations(&self) -> &HashMap<AntState, Animation> {
        &self.ant_animations
    }
}
