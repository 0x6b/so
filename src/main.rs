use std::{collections::BTreeMap, env, fmt, fmt::Display, path::PathBuf, process, str::FromStr};

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

    #[clap(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Parser)]
pub enum Command {
    /// Edit the configuration file with $EDITOR.
    Edit,
    /// Update the list of available channels in the configuration file.
    UpdateChannels {
        /// Slack API token. If not provided, it will be read from the SLACK_TOKEN environment
        /// variable.
        #[arg(short, long, env = "SLACK_TOKEN")]
        token: String,

        /// Generate a shell completion script after successfully updating the channels.
        #[arg(short, long)]
        generate_completion: bool,

        /// The shell to generate completion scripts for.
        #[arg(short, long, default_value = "fish")]
        shell: Shell,

        /// The path to write the completion script to.
        #[arg(short, long, default_value = "~/.config/fish/completions/so.fish")]
        path: String,
    },
    /// Generate a shell completion script. At the moment, only `fish` is supported.
    GenerateCompletion {
        /// The shell to generate completion scripts for.
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
    let Args { channel_name, browser, config, command } = Args::parse();
    let opener = SlackOpener::from(config).await?;

    match command {
        Some(Command::Edit) => {
            match process::Command::new(env::var("EDITOR").unwrap_or_else(|_| "vi".to_string()))
                .arg(&opener.path)
                .spawn()
            {
                Ok(mut child) => child.wait().map(|_| ()).map_err(Into::into),
                Err(e) => anyhow::bail!("Failed to open editor: {e}"),
            }
        }
        Some(Command::GenerateCompletion { shell: _, path }) => {
            generate_fish_completion(&opener, &path).await
        }
        Some(Command::UpdateChannels { token, generate_completion, shell: _, path }) => {
            update_channels(&opener, &token).await?;
            if generate_completion {
                generate_fish_completion(&opener, &path).await?;
            }
            Ok(())
        }
        None => match channel_name {
            Some(channel_name) => opener.open(&channel_name, browser),
            None => opener.open_prompt(browser),
        },
    }
}

async fn update_channels(opener: &SlackOpener, token: &str) -> Result<()> {
    println!("Updating channels.");
    let client = slack_client::ApiClient::new(&token)?;

    let mut results = vec![];
    let mut request = conversations::List {
        exclude_archived: Some(true),
        types: Some(
            vec![conversations::ChannelType::Public, conversations::ChannelType::Private].into(),
        ),
        cursor: None,
        limit: Some(1000),
    };

    loop {
        println!("Got {} channels", results.len());
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

    println!("Found {} channels, filtering out channels with no members", results.len());
    let channels = results
        .iter()
        .filter(|channel| channel.num_members.unwrap_or(1) > 0)
        .map(|channel| (ChannelName::from_str(channel.name.as_str()).unwrap(), channel.id.clone()))
        .collect::<BTreeMap<ChannelName, ChannelId>>();

    opener.update_config(channels).await
}

async fn generate_fish_completion(opener: &SlackOpener, path: &str) -> Result<()> {
    let file = PathBuf::from(tilde(&path).to_string());
    let mut out = BufWriter::new(File::create(&file).await?);

    out.write_all(b"# fish shell completions for so command\n").await?;
    out.write_all(b"complete -c so -f -n \"not __fish_seen_subcommand_from completion\"\n")
        .await?;
    out.write_all(
        b"complete -c so -f -n \"not __fish_seen_subcommand_from completion\" -a edit -d \"Edit the configuration file with $EDITOR\"\n",
    )
        .await?;
    out.write_all(
        b"complete -c so -f -n \"not __fish_seen_subcommand_from completion\" -a update-channels -d \"command: Update the list of available channels in the configuration file\"\n",
    )
        .await?;
    out.write_all(
        b"complete -c so -f -n \"not __fish_seen_subcommand_from completion\" -a generate-completion -d \"command: Generate a shell completion script\"\n",
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
    println!("Auto completion file updated: {}", file.display());

    Ok(())
}
