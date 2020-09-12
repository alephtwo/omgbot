import { Client } from "discord.js";

export default (client: Client) => (): void => {
    const username = client.user?.tag || '';
    console.log(`Logged in as ${username}!`);
}