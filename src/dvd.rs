use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    texture::Texture2D,
    RaylibHandle, RaylibThread,
};

pub struct Dvd {
    pub x: i32,
    pub y: i32,
    pub texture: Texture2D,
}

impl Dvd {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, texture_path: &str) -> Self {
        Self {
            x: 0,
            y: 0,
            texture: rl.load_texture(&thread, texture_path).unwrap(),
        }
    }

    pub fn update(&mut self) {}

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_texture(&self.texture, self.x, self.y, Color::WHITE);
    }
}
