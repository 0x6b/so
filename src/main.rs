use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use serde::Deserialize;
use toml::from_str;
use xdg::BaseDirectories;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub team_id: String,
    pub channels: HashMap<String, String>,
}

#[derive(Debug, Parser)]
#[clap(version)]
pub struct Args {
    /// The name of the channel to open
    #[arg()]
    pub channel_name: String,

    /// Path to the configuration file. Defaults to $XDG_CONFIG_HOME/slack-open/config.toml.
    #[arg(short, long)]
    pub config: Option<PathBuf>,
}

fn main() -> Result<()> {
    let Args { channel_name, config } = Args::parse();

    let path = config.unwrap_or_else(|| {
        BaseDirectories::with_prefix("slack-open")
            .unwrap()
            .place_config_file("config.toml")
            .unwrap()
    });
    let config = from_str::<Config>(&read_to_string(&path)?)?;

    match config.channels.get(&channel_name) {
        Some(id) => open::that(format!("slack://channel?id={id}&team={}", config.team_id))?,
        None => eprintln!("'{channel_name}' not found in '{path:?}'"),
    }

    Ok(())
}
