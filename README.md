# so

Open a Slack channel using the [deep link](https://api.slack.com/reference/deep-linking#supported_URIs) instead of the `https://` scheme.

## Usage

```console
$ so --help
Usage: so [OPTIONS] [CHANNEL_NAME]

Arguments:
  [CHANNEL_NAME]  The name of the channel to open. If not provided, select from a list of available channels

Options:
  -c, --config <CONFIG>  Path to the configuration file. Defaults to $XDG_CONFIG_HOME/so/config.toml
  -h, --help             Print help
  -V, --version          Print version
```

i.e.

```console
$ so random
```

or

```console
$ so # to select a channel from a list interactively
```

## Configuration

Place your configuration file at `$XDG_CONFIG_HOME/so/config.toml` or provide the path using the `--config` option.

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

MIT. See [LICENSE](LICENSE) for details.

