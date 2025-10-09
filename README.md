# omgbot

omgbot is a [Discord](https://discord.com) bot inspired by the Discord team's
original example bot, [airhorn.solutions](https://blog.discord.com/airhorn-solutions-the-only-discord-bot-youll-ever-need-c2d0ba7b92e8).

It provides a set of commands, each mapped to "short" sound clips.
When a command is issued, the following occurs:

1. A sound is chosen for the given command.
1. If the user issuing the command is in a voice channel, the bot joins that
   voice channel.
1. The bot plays the sound in the voice channel or uploads it to the text
   channel the user issued the command in.
1. If the bot joined a voice channel to play the sound, it leaves the channel.

## Usage

```shell
omgbot --discord-token "${your_token}" "${your_sound_dir}"
# or
DISCORD_TOKEN="${your_token}" omgbot "${your_sound_dir}"
```

### Configuration

Configuration is provided via command line arguments.

| Flag              | Description                                       |
| ----------------- | ------------------------------------------------- |
| `--discord-token` | The Discord token to authenticate with            |
| `--volume`        | The volume to play sounds at. (0-100, Default 25) |

## Development

1. Install [nextest](https://nexte.st/) and [cargo-mutants](https://mutants.rs/).
1. Run tests with `cargo nextest run`.
   1. Try to address any mutants by iterating with `cargo mutants`.
1. Ensure linting and formatting passes:
   1. `cargo clippy`
   1. `cargo fmt`
