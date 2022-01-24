import { Message } from 'discord.js';
import { parseBangCommand } from '../command/parseBangCommand';

export default (msg: Message): void => {
  // If it's not from a guild, don't bother doing anything.
  if (!msg.guild) {
    return;
  }

  const { command } = parseBangCommand(msg);
  if (!command) {
    return;
  }
  command.run();
};
