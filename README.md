# so

Open a Slack channel using [deep link](https://api.slack.com/reference/deep-linking#supported_URIs) i.e. `slack://...` or `https://app.slack.com/...` to prevent Slack from leaving the redirect tab open in your browser.

## Usage

```console
$ so --help
Usage: so [OPTIONS] [CHANNEL_NAME] [COMMAND]

Commands:
  update-channels      Update the list of available channels in the
                       configuration file
  generate-completion  Generate a shell completion script. At the moment, only
                       `fish` is supported
  help                 Print this message or the help of the given subcommand(s)

Arguments:
  [CHANNEL_NAME]  The name of the channel to open. If not provided, select from
                  a list of available channels

Options:
  -b, --browser          Open the channel in the browser instead of the Slack
                         app
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

`update-channels` subcommand can be used to update the channel list in the configuration file. Only `[channels]` section will be updated, so you can keep your aliases and team ID. 

If `--generate-completion` option is provided, a completion script will be generated after updating the channels. Provide `--shell` and `--path` options to customize the shell and the path to write the completion script.

```console
After updating the configuration file, you might want to regenerate the completion script.

```console
$ so update-channels --help
Update the list of available channels in the configuration file

Usage: so update-channels [OPTIONS] --token <TOKEN>

Options:
  -t, --token <TOKEN>        Slack API token. If not provided, it will be read
                             from the SLACK_TOKEN environment variable [env:
                             SLACK_TOKEN=...]
  -g, --generate-completion  Generate a shell completion script after
                             successfully updating the channels
  -s, --shell <SHELL>        The shell to generate completion scripts for
                             [default: fish]
  -p, --path <PATH>          The path to write the completion script to
                             [default: ~/.config/fish/completions/so.fish]
  -h, --help                 Print help
```

## Shell Completion

Completion script is available for the [fish](https://fishshell.com/).

```console
$ so generate-completion --help
Generate a shell completion script. At the moment, only `fish` is supported

Usage: so generate-completion [OPTIONS]

Options:
  -s, --shell <SHELL>  The shell to generate completion scripts for [default:
                       fish]
  -p, --path <PATH>    The path to write the completion script to [default:
                       ~/.config/fish/completions/so.fish]
  -h, --help           Print help
```

i.e.

```console
$ so generate-completion
```

## Note

As you can see, you cannot use the channel name `generate-completion` and `update-channels` as an argument. If you want to use it, provide an alias in the configuration file.

## License

MIT. See [LICENSE](LICENSE) for details.

