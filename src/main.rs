use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use slack_open::{ChannelName, SlackOpener};

#[derive(Debug, Parser)]
#[clap(version)]
pub struct Args {
    /// The name of the channel to open. If not provided, select from a list of available channels.
    #[arg()]
    pub channel_name: Option<ChannelName>,

    /// Path to the configuration file. Defaults to $XDG_CONFIG_HOME/slack-open/config.toml.
    #[arg(short, long)]
    pub config: Option<PathBuf>,
}

fn main() -> Result<()> {
    let Args { channel_name, config } = Args::parse();
    let opener = SlackOpener::from(config)?;

    if let Some(channel_name) = channel_name {
        opener.open(&channel_name)
    } else {
        opener.open_prompt()
    }
}
