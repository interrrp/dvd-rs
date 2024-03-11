use std::path::PathBuf;

#[derive(clap::Parser)]
#[command(version, about)]
pub struct Args {
    #[arg(default_value = "logo.png")]
    pub logo_path: PathBuf,
}
