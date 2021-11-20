import { Client } from 'discord.js';

export default (client: Client) => (): void => {
  const url = client.generateInvite({
    scopes: ['bot'],
  });
  console.log(`Invite URL: ${url}`);

  void client.user?.setActivity('!help for commands');
};
