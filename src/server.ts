import * as Discord from 'discord.js';
import onMessage from './events/onMessage';
import onReady from './events/onReady';
import { playSound } from './sound/playSound';
import { pickSound } from './sound/soundUtils';

const client = new Discord.Client({
  intents: [
    Discord.Intents.FLAGS.GUILDS,
    Discord.Intents.FLAGS.GUILD_MESSAGES,
    Discord.Intents.FLAGS.GUILD_VOICE_STATES,
  ],
});

client.on('ready', onReady(client));
client.on('messageCreate', onMessage);

// This code is extremely cursed.
// It is cursed code.
// Remove it or keep it at your own peril.
client.on('voiceStateUpdate', (prev, next) => {
  // Ignore when the bot itself joins the channel to prevent duplicates.
  if (next.member?.id === client.user?.id) {
    return;
  }

  if (!prev.channel && next.channel) {
    const channel = next.channel;
    // Set a timeout on this so it doesn't play THE INSTANT the user joins
    setTimeout(() => {
      void pickSound('hirys').then((sound) => playSound(channel, sound));
    }, 750);
  }
});

void client.login(process.env.DISCORD_TOKEN);
