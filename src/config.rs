use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub github: Vec<GithubEntry>,
}

#[derive(Debug, Deserialize)]
pub struct GithubEntry {
    pub repo: String,
}

pub fn read_config(config_str: &str) -> Result<Config> {
    let config: Config = toml::from_str(config_str).context("Failed to parse the configuration")?;

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_config_valid() {
        let config_str = r#"
            [[github]]
            repo = "YaLTeR/niri"
        "#;
        let config = read_config(config_str).expect("Failed to parse valid config");
        assert_eq!(config.github.len(), 1);
        assert_eq!(config.github[0].repo, "YaLTeR/niri");
    }
}
