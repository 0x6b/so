use std::{collections::BTreeMap, fmt, fmt::Display, path::PathBuf, str::FromStr};

use anyhow::{anyhow, Error, Result};
use clap::Parser;
use shellexpand::tilde;
use slack_client::{conversations, Response};
use so::{ChannelId, ChannelName, SlackOpener};
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

    /// Open the channel in the browser instead of the Slack app.
    #[arg(short, long)]
    pub browser: bool,

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
    /// Update the list of available channels in the configuration file.
    UpdateChannels {
        /// Slack API token. If not provided, it will be read from the SLACK_TOKEN environment
        /// variable.
        #[arg(short, long, env = "SLACK_TOKEN")]
        token: String,
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
    let Args { channel_name, browser, config, command } = Args::parse();
    let opener = SlackOpener::from(config).await?;

    match command {
        Some(Command::GenerateCompletion { shell: _, path }) => {
            let mut out =
                BufWriter::new(File::create(PathBuf::from(tilde(&path).to_string())).await?);

            out.write_all(b"# fish shell completions for so command\n").await?;
            out.write_all(b"complete -c so -f -n \"not __fish_seen_subcommand_from completion\"\n")
                .await?;
            out.write_all(
                b"complete -c so -f -n \"not __fish_seen_subcommand_from completion\" -a generate-completion -d \"command: Generate shell completion script\"\n",
            )
                .await?;
            out.write_all(
                b"complete -c so -f -n \"not __fish_seen_subcommand_from completion\" -a update-channels -d \"command: Update the list of available channels in the configuration file\"\n",
            )
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
        Some(Command::UpdateChannels { token }) => {
            let client = slack_client::ApiClient::new(&token)?;

            let mut results = vec![];
            let mut request = conversations::List {
                exclude_archived: Some(true),
                types: Some(
                    vec![conversations::ChannelType::Public, conversations::ChannelType::Private]
                        .into(),
                ),
                cursor: None,
                limit: Some(1000),
            };

            loop {
                let channels = client.conversations(&request).await?;
                let cursor = channels.next_cursor();

                if let Some(channels) = channels.channels {
                    results.extend(channels)
                }

                if cursor.is_some() {
                    request.cursor = cursor;
                } else {
                    break;
                }
            }
            results.sort_by(|a, b| a.name.cmp(&b.name));

            let channels = results
                .iter()
                .filter(|channel| channel.num_members.unwrap_or(0) > 0)
                .map(|channel| {
                    (ChannelName::from_str(channel.name.as_str()).unwrap(), channel.id.clone())
                })
                .collect::<BTreeMap<ChannelName, ChannelId>>();

            opener.update_config(channels).await?;
            Ok(())
        }
        None => match channel_name {
            Some(channel_name) => opener.open(&channel_name, browser),
            None => opener.open_prompt(browser),
        },
    }
}
