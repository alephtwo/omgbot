use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::anyhow;
use rand::{
    rng,
    seq::{IndexedRandom, IteratorRandom},
};

#[derive(Debug)]
pub struct Soundbank {
    data: HashMap<String, Vec<PathBuf>>,
}

pub struct SoundbankStats {
    pub categories: usize,
    pub sounds: usize,
    pub counts: HashMap<String, usize>,
}

impl Soundbank {
    pub fn new(sounds_dir: &Path) -> Result<Self, anyhow::Error> {
        let data = cache_soundbank(sounds_dir.to_path_buf())?;
        Ok(Self { data })
    }

    pub fn categories(&self) -> impl Iterator<Item = String> {
        self.data.keys().cloned()
    }

    pub fn stats(&self) -> SoundbankStats {
        let categories = self.data.keys().len();
        let counts: HashMap<String, usize> = self
            .data
            .iter()
            .map(|(k, v)| (k.clone(), v.len()))
            .collect();
        let sounds = counts.values().sum::<usize>();

        SoundbankStats {
            categories,
            sounds,
            counts,
        }
    }

    pub fn choose_sound(&self, category: &str) -> Result<PathBuf, anyhow::Error> {
        let sounds = self
            .data
            .get(category)
            .ok_or(anyhow!("No such category: {}", category))?;

        sounds
            .choose(&mut rng())
            .map(|p| p.to_path_buf())
            .ok_or(anyhow!("No sounds in category"))
    }

    pub fn choose_any_sound(&self) -> Result<PathBuf, anyhow::Error> {
        self.data
            .values()
            .flatten()
            .choose(&mut rng())
            .map(|p| p.to_path_buf())
            .ok_or(anyhow!("No sounds in soundbank"))
    }
}

fn cache_soundbank(sounds_dir: PathBuf) -> Result<HashMap<String, Vec<PathBuf>>, anyhow::Error> {
    let mut result: HashMap<String, Vec<PathBuf>> = HashMap::new();

    let categories = fs::read_dir(sounds_dir)?
        .filter_map(Result::ok)
        .filter(|e| e.path().is_dir());

    for category in categories {
        let category_name = category
            .file_name()
            .into_string()
            .map_err(|_| anyhow::anyhow!("Invalid category name"))?;

        let sound_files: Vec<PathBuf> = fs::read_dir(category.path())?
            .filter_map(Result::ok)
            .filter(|e| e.path().is_file())
            .map(|e| e.path())
            .collect();

        if !sound_files.is_empty() {
            result.insert(category_name, sound_files);
        }
    }

    Ok(result)
}
