use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("dvd-rs")
        .vsync()
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
    }
}
