use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use so::{ChannelName, SlackOpener};

#[derive(Debug, Parser)]
#[clap(version)]
pub struct Args {
    /// The name of the channel to open. If not provided, select from a list of available channels.
    #[arg()]
    pub channel_name: Option<ChannelName>,

    /// Path to the configuration file. Defaults to $XDG_CONFIG_HOME/so/config.toml.
    #[arg(short, long)]
    pub config: Option<PathBuf>,
}

fn main() -> Result<()> {
    let Args { channel_name, config } = Args::parse();
    let opener = SlackOpener::from(config)?;

    match channel_name {
        Some(channel_name) => opener.open(&channel_name),
        None => opener.open_prompt(),
    }
}
