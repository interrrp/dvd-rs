use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    texture::Texture2D,
    RaylibHandle, RaylibThread,
};

pub struct Dvd {
    pub x: f32,
    pub y: f32,
    pub vel_x: f32,
    pub vel_y: f32,
    pub texture: Texture2D,
}

impl Dvd {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, texture_path: &str) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            vel_x: 200.0,
            vel_y: 200.0,
            texture: rl.load_texture(&thread, texture_path).unwrap(),
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle) {
        let dt = rl.get_frame_time();
        self.x += self.vel_x * dt;
        self.y += self.vel_y * dt;
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_texture(&self.texture, self.x as i32, self.y as i32, Color::WHITE);
    }
}
