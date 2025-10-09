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
