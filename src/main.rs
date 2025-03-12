use anyhow::Context;
use clap::Parser;
use std::path::PathBuf;

mod config;

#[derive(Parser)]
#[command(name = "nv", about = "A utility to check new versions of software")]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let contents = if let Some(config_path) = cli.config {
        std::fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file at {:?}", config_path))?
    } else {
        let mut default_path = dirs::config_dir().expect("Could not determine config directory");
        default_path.push("nv/config.toml");
        std::fs::read_to_string(&default_path)
            .with_context(|| format!("Failed to read config file at {:?}", default_path))?
    };

    let config = config::read_config(&contents)?;
    println!("Config contents:\n{:#?}", config);
    Ok(())
}
