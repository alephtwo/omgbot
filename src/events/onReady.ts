import { Client } from 'discord.js';

export default (client: Client) => (): void => {
  void client.generateInvite().then((url) => {
    console.log(`Invite URL: ${url}`);
  });
};
