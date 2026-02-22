use std::time::Duration;

use ggez::graphics::{Image, Rect};

struct Animation {
    sprite_sheet: Image,
    sprite_frames: Vec<Rect>,
    time: Duration,
}
