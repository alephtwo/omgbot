import * as Discord from 'discord.js';
import onMessage from './events/onMessage';
import onReady from './events/onReady';

const client = new Discord.Client({
  intents: Discord.Intents.NON_PRIVILEGED
});

client.on('ready', onReady(client));
client.on('message', onMessage);

void client.login(process.env.DISCORD_TOKEN);
