import {
  AudioPlayerStatus,
  createAudioPlayer,
  createAudioResource,
  joinVoiceChannel,
  StreamType,
  VoiceConnectionStatus,
} from '@discordjs/voice';
import { StageChannel, VoiceChannel } from 'discord.js';

const player = createAudioPlayer();

export function playSound(channel: VoiceChannel | StageChannel, sound: string): void {
  const audio = createAudioResource(sound, { inputType: StreamType.Arbitrary });

  const connection = joinVoiceChannel({
    channelId: channel.id,
    guildId: channel.guild.id,
    adapterCreator: channel.guild.voiceAdapterCreator,
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
