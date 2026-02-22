use std::time::Duration;

use ggez::graphics::{Image, Rect};

#[allow(dead_code)]
struct Animation {
    sprite_sheet: Image,
    sprite_frames: Vec<Rect>,
    time: Duration,
}
