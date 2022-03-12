import { Message, MessageAttachment } from 'discord.js';
import { playSound } from '../../sound/playSound';
import Command from './Command';

export default abstract class PlaySoundCommand implements Command {
  #msg: Message;

  constructor(msg: Message) {
    this.#msg = msg;
  }

  abstract pickSound(): Promise<string>;

  async run() {
    const sound = await this.pickSound();

    // If the user isn't in a voice channel let's send them the file.
    const channel = this.#msg.member?.voice.channel;

    if (!channel) {
      const attachment = new MessageAttachment(sound);
      void this.#msg.channel.send({ files: [attachment] }).catch();
      return;
    }

    void playSound(channel, sound);
  }
}
