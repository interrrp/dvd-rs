use raylib::prelude::*;

mod dvd;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("dvd-rs")
        .vsync()
        .build();

    let mut dvd = dvd::Dvd::new(&mut rl, &thread, "logo.png");

    while !rl.window_should_close() {
        dvd.update();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        dvd.draw(&mut d);
    }
}
