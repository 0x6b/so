use std::collections::BTreeMap;

use serde::Deserialize;

use crate::{ChannelId, ChannelName};

/// Configuration for the Slack opener.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Slack team ID.
    pub team_id: String,

    /// Channel name aliases
    #[serde(deserialize_with = "deserialize_name_name")]
    pub aliases: Option<BTreeMap<ChannelName, ChannelName>>,

    /// Channel name to channel ID mappings.
    #[serde(deserialize_with = "deserialize_name_id")]
    pub channels: BTreeMap<ChannelName, ChannelId>,
}

fn deserialize_name_name<'de, D>(
    deserializer: D,
) -> Result<Option<BTreeMap<ChannelName, ChannelName>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(Some(
        BTreeMap::<String, String>::deserialize(deserializer)?
            .into_iter()
            .map(|(k, v)| (ChannelName { inner: k }, ChannelName { inner: v }))
            .collect(),
    ))
}

fn deserialize_name_id<'de, D>(
    deserializer: D,
) -> Result<BTreeMap<ChannelName, ChannelId>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(BTreeMap::<String, String>::deserialize(deserializer)?
        .into_iter()
        .map(|(k, v)| (ChannelName { inner: k }, v))
        .collect())
}
