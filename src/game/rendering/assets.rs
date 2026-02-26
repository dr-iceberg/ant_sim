use ggez::{
    GameResult,
    graphics::{GraphicsContext, Image, Rect},
};

pub struct Asset {
    img: Image,
}

impl Asset {
    pub fn new(gfx: &GraphicsContext, path: &str) -> Self {
        Asset {
            img: Image::from_path(gfx, path).unwrap(),
        }
    }

    pub fn img(&self) -> &Image {
        &self.img
    }
}

#[allow(dead_code)]
pub struct AssetManager {
    ant_sprite_sheet: Asset,
}

impl AssetManager {
    pub fn new(gfx: &GraphicsContext) -> GameResult<AssetManager> {
        let ant_img = Image::from_path(gfx, "/ant_sprite.png").unwrap();

        Ok(AssetManager {
            ant_sprite_sheet: Asset { img: ant_img },
        })
    }
    #[allow(dead_code)]
    pub fn ant_sprite_sheet(&self) -> &Asset {
        &self.ant_sprite_sheet
    }
}
