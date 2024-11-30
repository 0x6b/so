# so

Open a Slack channel using the [deep link](https://api.slack.com/reference/deep-linking#supported_URIs) i.e. `slack://`, instead of the `https://` to prevent the browser from opening the channel in a new tab.

## Usage

```console
$ so --help
Usage: so [OPTIONS] [CHANNEL_NAME] [COMMAND]

Commands:
  generate-completion  Generate shell completion script
  help                 Print this message or the help of the given subcommand(s)

Arguments:
  [CHANNEL_NAME]  The name of the channel to open. If not provided, select from a list
                  of available channels

Options:
  -c, --config <CONFIG>  Path to the configuration file. Defaults to
                         $XDG_CONFIG_HOME/so/config.toml
  -h, --help             Print help
  -V, --version          Print version
```

i.e.

```console
$ so random
```

or, run it without an argument to select a channel from a list interactively:

```console
$ so
```

If you are using the [fish](https://fishshell.com/) shell, you can generate a completion script using the `generate-completion` command.

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

## Shell Completion

Completion script is available for the [fish](https://fishshell.com/).

```console
$ so generate-completion --help
Generate shell completion script

Usage: so generate-completion [OPTIONS]

Options:
  -s, --shell <SHELL>  The shell to generate completion scripts for. At the moment,
                       only `fish` is supported [default: fish]
  -p, --path <PATH>    The path to write the completion script to [default:
                       ~/.config/fish/completions/so.fish]
  -h, --help           Print help
```

i.e.

```console
$ so generate-completion
```

which means you cannot use the channel name "generate-completion" as an argument. If you want to use it, provide an alias in the configuration file.


## License

MIT. See [LICENSE](LICENSE) for details.

