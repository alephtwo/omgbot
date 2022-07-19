import { Client, OAuth2Scopes } from 'discord.js';

export default (client: Client) => (): void => {
  const url = client.generateInvite({
    scopes: [OAuth2Scopes.Bot],
  });
  console.log(`Invite URL: ${url}`);

  void client.user?.setActivity('!help for commands');
};
