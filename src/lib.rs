mod config;
mod slack_opener;

use std::{borrow::Cow, fmt::Display, ops::Deref, str::FromStr};

pub use config::Config;
use serde::{Deserialize, Serialize};
use skim::SkimItem;
pub use slack_opener::SlackOpener;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct ChannelName {
    inner: String,
}

impl Deref for ChannelName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Display for ChannelName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl FromStr for ChannelName {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { inner: s.to_string() })
    }
}

impl From<Cow<'_, str>> for ChannelName {
    fn from(s: Cow<'_, str>) -> Self {
        Self { inner: s.into_owned() }
    }
}

pub type ChannelId = String;

impl SkimItem for ChannelName {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(self)
    }
}
