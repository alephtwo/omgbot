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
        value_parser = clap::value_parser!(u16).range(0..=100),
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

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use tempfile::TempDir;

    #[test]
    fn dir_does_not_exist() {
        assert_eq!(
            validate_dir_exists("8626a752-24c9-491f-98ee-579e63bb64e4"),
            Err("Directory doesn't exist: 8626a752-24c9-491f-98ee-579e63bb64e4".into())
        );
    }

    #[test]
    fn path_is_not_a_directory() -> Result<(), anyhow::Error> {
        // Create a directory containing a temporary file
        let tmp = TempDir::with_suffix("omgbot-test-cli")?;
        let path = &tmp.path().join("test.txt");
        File::create(path)?;

        // Confirm that the file is - indeed - not a directory
        let expected_path = path
            .to_str()
            .ok_or(anyhow::anyhow!("non-utf8 strings not allowed"))?;
        assert_eq!(
            validate_dir_exists(expected_path),
            Err(format!("Not a directory: {}", expected_path))
        );

        // Close the temp directory
        tmp.close()?;
        Ok(())
    }
}
