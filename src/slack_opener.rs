use std::{fs::read_to_string, ops::Deref, path::PathBuf, sync::Arc};

use anyhow::{anyhow, Result};
use skim::{
    options::SkimOptionsBuilder, prelude::unbounded, Skim, SkimItemReceiver, SkimItemSender,
};

use crate::{ChannelId, ChannelName, Config};

/// A Slack channel opener, which ...opens a Slack channel directly (means it opens the Slack
/// channel without opening a browser).
pub struct SlackOpener {
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
    pub fn from(config_path: Option<PathBuf>) -> Result<Self> {
        let config = Self::parse_config(config_path)?;
        Ok(Self { config })
    }

    /// Open the Slack channel in the default browser.
    ///
    /// # Arguments
    ///
    /// - `channel_name` - The name of the channel to open.
    pub fn open(&self, name: &ChannelName) -> Result<()> {
        let id = self
            .get_channel_id(name)
            .ok_or_else(|| anyhow!("Channel not found: {name}"))?;

        // The `slack://` URI scheme is supported by the Slack desktop app.
        //
        // See also: https://api.slack.com/reference/deep-linking#open_a_channel
        open::that(format!("slack://channel?team={}&id={id}", self.team_id)).map_err(Into::into)
    }

    pub fn open_prompt(&self) -> Result<()> {
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
            Some(out) if out.is_abort => return Ok(()),
            Some(out) => {
                let name = out
                    .selected_items
                    .first()
                    .ok_or_else(|| anyhow!("No channel selected"))?
                    .as_ref()
                    .text();
                self.open(&name.into())
            }
            None => Ok(()),
        }
    }

    fn parse_config(path: Option<PathBuf>) -> Result<Config> {
        let path = path.unwrap_or_else(|| {
            xdg::BaseDirectories::with_prefix("sopen")
                .unwrap()
                .place_config_file("config.toml")
                .unwrap()
        });

        Ok(toml::from_str::<Config>(&read_to_string(&path)?)?)
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
