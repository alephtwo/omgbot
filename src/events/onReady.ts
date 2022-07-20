import { Client, OAuth2Scopes } from 'discord.js';

export function onReady(client: Client) {
  return () => {
    const url = client.generateInvite({
      scopes: [OAuth2Scopes.Bot],
    });
    console.log(`Invite URL: ${url}`);

    void client.user?.setActivity('!help for commands');
  };
}
