import * as Discord from 'discord.js';
import onMessage from './events/onMessage';
import onReady from './events/onReady';

const client = new Discord.Client();
client.on('ready', onReady(client));
client.on('message', onMessage);
client.login(process.env.DISCORD_TOKEN);