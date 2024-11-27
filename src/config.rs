use std::collections::HashMap;

use serde::Deserialize;

use crate::{ChannelId, ChannelName};

/// Configuration for the Slack opener.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Slack team ID.
    pub team_id: String,

    /// Channel name to channel ID mappings.
    pub channels: HashMap<ChannelName, ChannelId>,

    /// Channel name aliases
    pub aliases: Option<HashMap<ChannelName, ChannelName>>,
}
