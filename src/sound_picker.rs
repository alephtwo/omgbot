use rand::{seq::SliceRandom, thread_rng};
use serenity::voice::{ffmpeg, AudioSource};
use std::{env, fs, path::PathBuf};

pub fn pick(category: String) -> Option<Box<dyn AudioSource>> {
    let path = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("sounds")
        .join(&category);

    let paths: Vec<PathBuf> = fs::read_dir(path)
        .unwrap()
        .map(|p| p.unwrap().path())
        .collect();

    let path = match paths.choose(&mut thread_rng()) {
        Some(p) => p,
        None => {
            eprintln!("No paths");
            return None;
        }
    };

    match ffmpeg(path) {
        Ok(source) => Some(source),
        Err(why) => {
            eprintln!("Error picking source: {:?}", why);
            return None;
        }
    }
}
