use ggez::{
    GameResult,
    graphics::{GraphicsContext, Image, Rect},
};

pub struct AssetManager {
    ant_png: Image,
    ant_img: Image,
    ant_sprites: Vec<Rect>,
}

impl AssetManager {
    pub fn new(gfx: &GraphicsContext) -> GameResult<AssetManager> {
        let ant_img = Image::from_path(gfx, "/ant_sprite.png").unwrap();
        let ant_png = Image::from_path(gfx, "/ant.png").unwrap();

        Ok(AssetManager {
            ant_png: ant_png,
            ant_img: ant_img,
            ant_sprites: Vec::new(),
        })
    }

    pub fn ant_img(&self) -> &Image {
        &self.ant_img
    }

    pub fn ant_png(&self) -> &Image {
        &self.ant_png
    }
}
