import { Client } from 'discord.js';

export default (client: Client) => (): void => {
  const url = client.generateInvite({
    // TODO: this should just be a bot, but it won't compile like that
    scopes: ['applications.commands'],
  });
  console.log(`Invite URL: ${url}`);

  void client.user?.setActivity('!help for commands');
};
