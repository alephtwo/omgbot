use rand::{seq::SliceRandom, thread_rng};
use serenity::voice::{ffmpeg, AudioSource};
use std::{env, fs, path::PathBuf};

pub fn pick_file(category: String) -> Option<PathBuf> {
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

    let result = paths.choose(&mut thread_rng());

    match result {
        Some(t) => Some(t.to_path_buf()),
        None => None
    }
}

pub fn to_audio_source(path: &PathBuf) -> Option<Box<dyn AudioSource>> {
    match ffmpeg(path) {
        Ok(source) => Some(source),
        Err(why) => {
            eprintln!("Error picking source: {:?}", why);
            return None;
        }
    }
}
