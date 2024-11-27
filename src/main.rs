use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use slack_open::{ChannelName, SlackOpener};

#[derive(Debug, Parser)]
#[clap(version)]
pub struct Args {
    /// The name of the channel to open
    #[arg()]
    pub channel_name: ChannelName,

    /// Path to the configuration file. Defaults to $XDG_CONFIG_HOME/slack-open/config.toml.
    #[arg(short, long)]
    pub config: Option<PathBuf>,
}

fn main() -> Result<()> {
    let Args { channel_name, config } = Args::parse();

    SlackOpener::from(config)?.open(&channel_name)
}
