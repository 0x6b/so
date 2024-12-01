use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{ChannelId, ChannelName};

/// Configuration for the Slack opener.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Slack team ID.
    pub team_id: String,

    /// Channel name aliases
    #[serde(serialize_with = "serialize_name_name", deserialize_with = "deserialize_name_name")]
    pub aliases: Option<BTreeMap<ChannelName, ChannelName>>,

    /// Channel name to channel ID mappings.
    #[serde(serialize_with = "serialize_name_id", deserialize_with = "deserialize_name_id")]
    pub channels: BTreeMap<ChannelName, ChannelId>,
}

fn serialize_name_name<S>(
    value: &Option<BTreeMap<ChannelName, ChannelName>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value {
        Some(v) => v
            .iter()
            .map(|(k, v)| (k.inner.as_str(), v.inner.as_str()))
            .collect::<BTreeMap<&str, &str>>()
            .serialize(serializer),
        None => serializer.serialize_none(),
    }
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

fn serialize_name_id<S>(
    value: &BTreeMap<ChannelName, ChannelId>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    value
        .iter()
        .map(|(k, v)| (k.inner.as_str(), v.as_str()))
        .collect::<BTreeMap<&str, &str>>()
        .serialize(serializer)
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
