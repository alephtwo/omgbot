import { Message, MessageAttachment } from 'discord.js';
import { parseBangCommand } from '../command/parseBangCommand';
import { pickSound, getAllCategories } from '../sound/soundUtils';
import { playSound } from '../sound/playSound';

const categories = getAllCategories();

export default (msg: Message): void => {
  // If it's not from a guild, don't bother doing anything.
  if (!msg.guild) {
    return;
  }

  const command = parseBangCommand(msg.content);
  if (command === null) {
    return;
  }
  // TODO: Improve this... see parseBangCommand.ts
  if (command === 'help') {
    displayHelp(msg);
  }
  if (!categories.has(command)) {
    return;
  }

  // Pick a sound...
  const sound = pickSound(command);

  // If the user isn't in a voice channel let's send them the file.
  const channel = msg.member?.voice.channel;

  if (!channel) {
    const attachment = new MessageAttachment(sound);
    void msg.channel.send({ files: [attachment] }).catch();
    return;
  }

  void playSound(channel, sound);
};

function displayHelp(msg: Message) {
  const help = Array.from(categories)
    .sort((a: string, b: string) => a.localeCompare(b))
    .map((c) => `* \`!${c}\``)
    .join('\n');

  void msg.author.send(help).catch();
}
