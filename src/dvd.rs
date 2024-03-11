use std::path::PathBuf;

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
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, texture_path: &PathBuf) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            vel_x: 250.0,
            vel_y: 250.0,
            texture: rl
                .load_texture(&thread, texture_path.to_str().unwrap())
                .unwrap(),
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle) {
        if self.x <= 0.0 || self.x + self.texture.width as f32 >= rl.get_screen_width() as f32 {
            // Hit either left or right edge, reverse horizontal velocity
            self.vel_x *= -1.0;
        }
        if self.y <= 0.0 || self.y + self.texture.height as f32 >= rl.get_screen_height() as f32 {
            // Hit either top or bottom edge, reverse vertical velocity
            self.vel_y *= -1.0;
        }

        let dt = rl.get_frame_time();
        self.x += self.vel_x * dt;
        self.y += self.vel_y * dt;
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_texture(&self.texture, self.x as i32, self.y as i32, Color::WHITE);
    }
}
