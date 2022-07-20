import { Message } from 'discord.js';
import { parseBangCommand } from '../command/parseBangCommand';

export async function onMessageCreate(msg: Message): Promise<void> {
  // If it's not from a guild, don't bother doing anything.
  if (!msg.guild) {
    return;
  }

  const { command } = parseBangCommand(msg);
  if (!command) {
    return;
  }
  await command.run();
}
