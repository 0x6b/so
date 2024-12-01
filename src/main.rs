use std::{fmt, fmt::Display, path::PathBuf, str::FromStr};

use anyhow::{anyhow, Error, Result};
use clap::Parser;
use shellexpand::tilde;
use so::{ChannelName, SlackOpener};
use tokio::{
    fs::File,
    io::{AsyncWriteExt, BufWriter},
};

#[derive(Debug, Parser)]
#[clap(version)]
pub struct Args {
    /// The name of the channel to open. If not provided, select from a list of available channels.
    #[arg()]
    pub channel_name: Option<ChannelName>,

    /// Path to the configuration file. Defaults to $XDG_CONFIG_HOME/so/config.toml.
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    /// Generate shell completion scripts.
    #[clap(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Parser)]
pub enum Command {
    /// Generate shell completion script.
    GenerateCompletion {
        /// The shell to generate completion scripts for. At the moment, only `fish` is supported.
        #[arg(short, long, default_value = "fish")]
        shell: Shell,

        /// The path to write the completion script to.
        #[arg(short, long, default_value = "~/.config/fish/completions/so.fish")]
        path: String,
    },
}

#[derive(Debug, Clone)]
pub enum Shell {
    Fish,
}

impl Display for Shell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fish => write!(f, "fish"),
        }
    }
}

impl FromStr for Shell {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "fish" => Ok(Self::Fish),
            _ => Err(anyhow!("Unsupported shell: {s}")),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let Args { channel_name, config, command } = Args::parse();
    let opener = SlackOpener::from(config).await?;

    match command {
        Some(Command::GenerateCompletion { shell: _, path }) => {
            let mut out =
                BufWriter::new(File::create(PathBuf::from(tilde(&path).to_string())).await?);

            out.write_all(b"# fish shell completions for so command\n").await?;
            out.write_all(b"complete -c so -f -n \"not __fish_seen_subcommand_from completion\"\n")
                .await?;
            for name in opener.channels.iter().map(|c| c.0) {
                out.write_all(
                    format!(
                        "complete -c so -f -n \"not __fish_seen_subcommand_from completion\" -a \"{name}\" -d \"#{name}\"\n",
                    )
                        .as_bytes(),
                )
                    .await?;
            }

            out.flush().await?;
            Ok(())
        }
        None => match channel_name {
            Some(channel_name) => opener.open(&channel_name),
            None => opener.open_prompt(),
        },
    }
}
