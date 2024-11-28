use std::collections::HashMap;

use serde::Deserialize;

use crate::{ChannelId, ChannelName};

/// Configuration for the Slack opener.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Slack team ID.
    pub team_id: String,

    /// Channel name aliases
    #[serde(deserialize_with = "deserialize_name_name")]
    pub aliases: Option<HashMap<ChannelName, ChannelName>>,

    /// Channel name to channel ID mappings.
    #[serde(deserialize_with = "deserialize_name_id")]
    pub channels: HashMap<ChannelName, ChannelId>,
}

fn deserialize_name_name<'de, D>(
    deserializer: D,
) -> Result<Option<HashMap<ChannelName, ChannelName>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(Some(
        HashMap::<String, String>::deserialize(deserializer)?
            .into_iter()
            .map(|(k, v)| (ChannelName { inner: k }, ChannelName { inner: v }))
            .collect(),
    ))
}

fn deserialize_name_id<'de, D>(deserializer: D) -> Result<HashMap<ChannelName, ChannelId>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(HashMap::<String, String>::deserialize(deserializer)?
        .into_iter()
        .map(|(k, v)| (ChannelName { inner: k }, v))
        .collect())
}
