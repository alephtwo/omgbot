use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(long, env = "DISCORD_TOKEN")]
    pub discord_token: String,
    #[arg(
        name = "sounds",
        value_hint = clap::ValueHint::DirPath,
        value_parser = validate_dir_exists
    )]
    pub sounds_dir: PathBuf,
    #[arg(
        long,
        value_parser = clap::value_parser!(u16).range(0..100),
        default_value_t = 25
    )]
    pub volume: u16,
}

fn validate_dir_exists(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    if !path.exists() {
        return Err(format!("Directory doesn't exist: {s}"));
    }
    if !path.is_dir() {
        return Err(format!("Not a directory: {s}"));
    }
    Ok(path)
}
