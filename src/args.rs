use std::path::PathBuf;

#[derive(clap::Parser)]
#[command(version, about)]
pub struct Args {
    #[arg(default_value = "logo.png")]
    pub logo_path: PathBuf,

    #[arg(short, long, default_value_t = 250.0)]
    pub x_vel: f32,
    #[arg(short, long, default_value_t = 250.0)]
    pub y_vel: f32,
}
