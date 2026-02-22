use ggez::glam::Vec2;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Food {
    pos: Vec2,
    energy: u32,
}

#[allow(dead_code)]
impl Food {
    pub fn new(pos: Vec2) -> Self {
        Food {
            pos: pos,
            energy: 50,
        }
    }

    pub fn pos(&self) -> Vec2 {
        self.pos
    }

    pub fn energy(&self) -> u32 {
        self.energy
    }
}
