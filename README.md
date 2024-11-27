# slack-open

Open a Slack channel using the [deep link](https://api.slack.com/reference/deep-linking#supported_URIs) instead of the `https://` scheme.

## Usage

```console
$ sopen --help
```

## Configuration

Place your configuration file at `$XDG_CONFIG_HOME/sopen/config.toml`, or provide the path using the `--config` option.

```toml
# Slack team ID
team_id = "Txxxxxxxx"

# Channel name aliases
[aliases]
"r" = "random"
# ...
 
# Map of Slack channel name or nickname to its ID
[channels]
"channel_name" = "channel ID"
# ...
```

## License

MIT. See [LICENSE](LICENSE).
