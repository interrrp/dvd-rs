use clap::Parser;
use raylib::prelude::*;

mod args;
mod dvd;

fn main() {
    let args = args::Args::parse();

    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("dvd-rs")
        .vsync()
        .build();

    let mut dvd = dvd::Dvd::new(&mut rl, &thread, args.x_vel, args.y_vel, &args.logo_path);

    while !rl.window_should_close() {
        dvd.update(&mut rl);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        dvd.draw(&mut d);
    }
}
