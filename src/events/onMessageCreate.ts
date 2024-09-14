import { parseBangCommand } from "../command/parseBangCommand";
import { Message } from "discord.js";

export function onMessageCreate(msg: Message) {
  // If it's not from a guild, don't bother doing anything.
  if (!msg.guild) {
    return;
  }

  const { command } = parseBangCommand(msg);
  if (!command) {
    return;
  }
  command.run();
}
