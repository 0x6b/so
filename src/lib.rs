mod config;
mod slack_opener;

pub use config::Config;
pub use slack_opener::SlackOpener;

pub type ChannelName = String;
pub type ChannelId = String;
