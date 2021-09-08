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

client.on('voiceStateUpdate', (_prev, next) => {
  if (next.channel) {
    void playSound(next.channel, pickSound('hirys'));
  }
});

void client.login(process.env.DISCORD_TOKEN);
