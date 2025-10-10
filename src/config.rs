use crate::{cli::Cli, soundbank::Soundbank};

#[derive(Debug)]
pub struct BotConfig {
    pub soundbank: Soundbank,
    pub volume: f32,
}

impl BotConfig {
    pub fn from_cli(cli: Cli) -> Result<Self, anyhow::Error> {
        let volume = f32::from(cli.volume) / 100.0;

        let soundbank = Soundbank::new(&cli.sounds_dir)?;
        let stats = soundbank.stats();
        tracing::info!(
            "Loaded {} total sounds in {} categories",
            stats.sounds,
            stats.categories
        );

        Ok(Self { soundbank, volume })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::Cli;
    use std::fs::File;
    use tempfile::TempDir;

    #[test]
    fn test_load_config() -> Result<(), anyhow::Error> {
        // Make some sounds
        // Create a directory containing a temporary file
        let tmp = TempDir::with_suffix("omgbot-test-config")?;
        // one category
        let cat1 = &tmp.path().join("cat1");
        std::fs::create_dir(cat1)?;
        File::create(cat1.join("sound1.mp3"))?;
        File::create(cat1.join("sound2.mp3"))?;
        // another category
        let cat2 = &tmp.path().join("cat2");
        std::fs::create_dir(cat2)?;
        File::create(cat2.join("sound3.mp3"))?;

        let cli = Cli {
            discord_token: "f4b148db-8af5-48b2-a248-6415e404a92f".to_string(),
            sounds_dir: tmp.path().to_path_buf(),
            volume: 75,
        };

        let config = BotConfig::from_cli(cli).expect("Failed to load config");
        assert_eq!(config.volume, 0.75);
        assert_eq!(config.soundbank.categories().count(), 2);
        assert_eq!(config.soundbank.stats().sounds, 3);
        Ok(())
    }
}
