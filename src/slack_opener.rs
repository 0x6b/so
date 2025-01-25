use std::{collections::BTreeMap, ops::Deref, path::PathBuf, sync::Arc};

use anyhow::{anyhow, Result};
use skim::{
    options::SkimOptionsBuilder, prelude::unbounded, Skim, SkimItemReceiver, SkimItemSender,
};
use tokio::fs::{read_to_string, write};

use crate::{ChannelId, ChannelName, Config};

/// A Slack channel opener, which ...opens a Slack channel directly (means it opens the Slack
/// channel without opening a browser).
pub struct SlackOpener {
    /// Resolved path to the configuration file.
    pub path: PathBuf,

    /// The configuration.
    config: Config,
}

impl Deref for SlackOpener {
    type Target = Config;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}

impl SlackOpener {
    /// Create a new `SlackOpener` instance.
    ///
    /// # Arguments
    ///
    /// - `config` - Path to the configuration file.
    pub async fn from(config_path: Option<PathBuf>) -> Result<Self> {
        let (path, config) = Self::parse_config(config_path).await?;
        Ok(Self { path, config })
    }

    /// Open the Slack channel in the default browser.
    ///
    /// # Arguments
    ///
    /// - `channel_name` - The name of the channel to open.
    /// - `browser` - Whether to open the channel in the browser or a Slack app.
    pub fn open(&self, name: &ChannelName, browser: bool) -> Result<()> {
        let id = self
            .get_channel_id(name)
            .ok_or_else(|| anyhow!("Channel not found: {name}"))?;

        if browser {
            open::that(format!("https://app.slack.com/client/{}/{id}", self.team_id))
                .map_err(Into::into)
        } else {
            // The `slack://` URI scheme is supported by the Slack desktop app.
            //
            // See also: https://api.slack.com/reference/deep-linking#open_a_channel
            open::that(format!("slack://channel?team={}&id={id}", self.team_id)).map_err(Into::into)
        }
    }

    /// Open an interactive prompt to select a channel to open.
    ///
    /// # Arguments
    ///
    /// - `browser` - Whether to open the channel in the browser or a Slack app.
    pub fn open_prompt(&self, browser: bool) -> Result<()> {
        let options = SkimOptionsBuilder::default()
            .height(String::from("5"))
            .multi(false)
            .build()?;

        let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
        self.channels
            .keys()
            .map(|name| Arc::new(name.clone()))
            .for_each(|item| {
                let _ = tx_item.send(item);
            });
        drop(tx_item); // so that skim could know when to stop waiting for more items.

        match Skim::run_with(&options, Some(rx_item)) {
            Some(out) if out.is_abort => Ok(()),
            Some(out) if !out.selected_items.is_empty() => {
                self.open(&out.selected_items.first().unwrap().text().into(), browser)
            }
            _ => Ok(()),
        }
    }

    async fn parse_config(path: Option<PathBuf>) -> Result<(PathBuf, Config)> {
        let path = path.unwrap_or_else(|| {
            xdg::BaseDirectories::with_prefix("so")
                .unwrap()
                .place_config_file("config.toml")
                .unwrap()
        });
        let config = toml::from_str::<Config>(&read_to_string(&path).await?)?;

        Ok((path, config))
    }

    /// Update the configuration file with the provided map of channels.
    ///
    /// # Arguments
    ///
    /// - `channels` - A map of channel names to channel IDs.
    pub async fn update_config(&self, channels: BTreeMap<ChannelName, ChannelId>) -> Result<()> {
        let len = channels.len();

        println!("Number of channels: {} → {}", self.channels.len(), len);

        let new_channels = channels
            .keys()
            .filter(|name| !self.channels.contains_key(name))
            .collect::<Vec<_>>();
        if !new_channels.is_empty() {
            println!("New channel(s):");
            new_channels.iter().for_each(|name| println!("  #{name}"));
        }

        let removed_channels = self
            .channels
            .keys()
            .filter(|name| !channels.contains_key(name))
            .collect::<Vec<_>>();
        if !removed_channels.is_empty() {
            println!("Removed channel(s):");
            removed_channels.iter().for_each(|name| println!("  #{name}"));
        }

        write(&self.path, toml::to_string(&Config { channels, ..self.config.clone() })?).await?;
        println!("Configuration file updated: {}", self.path.display());
        Ok(())
    }

    fn get_channel_id(&self, name: &ChannelName) -> Option<ChannelId> {
        self.get_channel_id_from_alias(name)
            .or_else(|| self.get_channel_id_from_channel_name(name))
    }

    fn get_channel_id_from_channel_name(&self, name: &ChannelName) -> Option<ChannelId> {
        self.channels.get(name).cloned()
    }

    fn get_channel_id_from_alias(&self, name: &ChannelName) -> Option<ChannelId> {
        self.aliases
            .as_ref()
            .and_then(|aliases| aliases.get(name).cloned())
            .and_then(|name| self.get_channel_id_from_channel_name(&name))
    }
}
