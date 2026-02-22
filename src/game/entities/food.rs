use ggez::glam::Vec2;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Food {
    pos: Vec2,
    energy: u32,
}

#[allow(dead_code)]
impl Food {
    pub fn new() -> Self {
        Food {
            pos: Vec2 { x: 100., y: 100. },
            energy: 50,
        }
    }
}
