use std::{
    fmt,
    fmt::Display,
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
    str::FromStr,
};

use anyhow::{anyhow, Error, Result};
use clap::Parser;
use shellexpand::tilde;
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

fn main() -> Result<()> {
    let Args { channel_name, config, command } = Args::parse();
    let opener = SlackOpener::from(config)?;

    match command {
        Some(Command::GenerateCompletion { shell: _, path }) => {
            let mut out = BufWriter::new(File::create(PathBuf::from(tilde(&path).to_string()))?);

            writeln!(out, "# Channel name completions for so command")?;
            writeln!(out, r#"complete -c so -f -n "not __fish_seen_subcommand_from completion""#)?;
            opener.channels.iter().map(|c| c.0).for_each(|name| {
                writeln!(out, r##"complete -c so -f -n "not __fish_seen_subcommand_from completion" -a "{name}" -d "#{name}""##).unwrap();
            });

            Ok(())
        }
        None => match channel_name {
            Some(channel_name) => opener.open(&channel_name),
            None => opener.open_prompt(),
        },
    }
}
