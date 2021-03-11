import { Message, MessageAttachment, VoiceChannel } from 'discord.js';
import { pickSound, getAllCategories } from '../sounds';

const categories = getAllCategories();

export default (msg: Message): void => {
  // If it's not from a guild, don't bother doing anything.
  if (!msg.guild) {
    return;
  }

  // If this isn't a command, we can stop.
  if (!msg.content.startsWith('!')) {
    return;
  }

  // Make sure it's a real category
  const category = msg.content.replace(/^!/, '');
  if (category === 'help') {
    displayHelp(msg);
    return;
  }

  if (!categories.has(category)) {
    return;
  }

  // Pick a sound...
  const sound = pickSound(category);

  // If the user isn't in a voice channel let's send them the file.
  const channel = msg.member?.voice.channel;
  if (!channel) {
    const attachment = new MessageAttachment(sound);
    void msg.channel.send(attachment);
    return;
  }

  void playSound(channel, sound);
};

async function playSound(channel: VoiceChannel, sound: string) {
  const conn = await channel.join();

  // Need to add quotes around the sound so that it doesn't break when
  // the path contains spaces... big sigh.
  const dispatcher = conn.play(`"${sound}"`);

  console.log(sound);

  dispatcher.on('finish', () => {
    dispatcher.destroy();
    conn.disconnect();
  });

  dispatcher.on('error', (error) => {
    console.error(error);
    conn.disconnect();
  });
}

function displayHelp(msg: Message) {
  const help = Array.from(categories)
    .sort((a: string, b: string) => a.localeCompare(b))
    .map((c) => `* \`!${c}\``);

  void msg.author.send(help);
}
