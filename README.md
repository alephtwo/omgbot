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

Set your API token as the expected environment variable:

```bash
DISCORD_TOKEN=your-api-key
```

Then run `npm start`.

`start.sh` accomplishes the same task but endeavors to make doing so easier on
a production box.

## Development Notes

1. Ensure that `npm run lint` doesn't report any issues.
