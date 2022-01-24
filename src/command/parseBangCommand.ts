import { Message } from 'discord.js';
import { getAllCategories } from '../sound/soundUtils';
import Command from './commands/Command';
import HelpCommand from './commands/HelpCommand';
import { InvalidCommand } from './commands/InvalidCommand';
import PlayRandomSoundCommand from './commands/PlayRandomSoundCommand';
import PlaySoundFromCategoryCommand from './commands/PlaySoundFromCategoryCommand';

const categories = getAllCategories();

export function parseBangCommand(msg: Message): { parsed?: string; command?: Command } {
  // Find if there is any string that might be a command
  const commands = msg.content.split(' ').filter((c) => c.startsWith('!'));

  // If there aren't any commands, just bail out
  if (commands.length === 0) {
    return {};
  }

  // Grab the first command, strip the bang
  const command = commands[0].replace(/^!/, '').trim();

  switch (command) {
    case '':
      return { parsed: command, command: new PlayRandomSoundCommand(msg) };
    case 'help':
      return { parsed: command, command: new HelpCommand(msg) };
    default:
      return { parsed: command, command: playSoundIfCategoryExists(command, msg) };
  }
}

function playSoundIfCategoryExists(command: string, msg: Message) {
  if (categories.has(command)) {
    return new PlaySoundFromCategoryCommand(msg, command);
  }
  return new InvalidCommand(command);
}
