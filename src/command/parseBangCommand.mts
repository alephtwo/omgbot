import { Message } from "discord.js";
import { getAllCategories } from "../sound/soundUtils.mjs";
import Command from "./commands/Command.mjs";
import HelpCommand from "./commands/HelpCommand.mjs";
import { InvalidCommand } from "./commands/InvalidCommand.mjs";
import PlayRandomSoundCommand from "./commands/PlayRandomSoundCommand.mjs";
import PlaySoundFromCategoryCommand from "./commands/PlaySoundFromCategoryCommand.mjs";
import StatsCommand from "./commands/StatsCommand.mjs";

const categories = getAllCategories();

export function parseBangCommand(msg: Message): {
  parsed?: string;
  command?: Command;
} {
  // Find if there is any string that might be a command
  const commands = msg.content.split(" ").filter((c) => c.startsWith("!"));

  // If there aren't any commands, just bail outa
  if (commands.length === 0) {
    return {};
  }

  // Grab the first command, strip the bang
  const command = commands[0].replace(/^!/, "").trim();

  switch (command) {
    case "":
      return { parsed: command, command: new PlayRandomSoundCommand(msg) };
    case "help":
      return { parsed: command, command: new HelpCommand(msg) };
    case "stats":
      return { parsed: command, command: new StatsCommand(msg) };
    default:
      return {
        parsed: command,
        command: playSoundIfCategoryExists(command, msg),
      };
  }
}

function playSoundIfCategoryExists(command: string, msg: Message) {
  if (categories.has(command)) {
    return new PlaySoundFromCategoryCommand(msg, command);
  }
  return new InvalidCommand(msg, command);
}
