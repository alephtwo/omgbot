import {
  AudioPlayerStatus,
  createAudioPlayer,
  createAudioResource,
  DiscordGatewayAdapterCreator,
  joinVoiceChannel,
  StreamType,
  VoiceConnectionStatus,
} from '@discordjs/voice';
import { Message, MessageAttachment, StageChannel, VoiceChannel } from 'discord.js';
import { pickSound, getAllCategories } from '../sounds';

const categories = getAllCategories();
const player = createAudioPlayer();

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
    msg.channel.send({ files: [attachment] }).catch();
    return;
  }

  void playSound(channel, sound);
};

async function playSound(channel: VoiceChannel | StageChannel, sound: string) {
  const audio = createAudioResource(sound, { inputType: StreamType.Arbitrary });

  const connection = joinVoiceChannel({
    channelId: channel.id,
    guildId: channel.guild.id,
    // TODO: Remove this cast, it probably doesn't even work
    adapterCreator: channel.guild.voiceAdapterCreator as DiscordGatewayAdapterCreator,
  });
  connection.subscribe(player);

  player.play(audio);
  player.on('error', (error) => {
    console.log(error);
    connection.destroy();
  });

  player.on('stateChange', (_prev, next) => {
    if (next.status === AudioPlayerStatus.Idle && connection.state.status !== VoiceConnectionStatus.Destroyed) {
      connection.destroy();
    }
  });
}

function displayHelp(msg: Message) {
  const help = Array.from(categories)
    .sort((a: string, b: string) => a.localeCompare(b))
    .map((c) => `* \`!${c}\``)
    .join('\n');

  void msg.author.send(help).catch();
}
