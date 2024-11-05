import { Message, AttachmentBuilder, ChannelType } from "discord.js";
import { playSound } from "../../sound/playSound.mjs";
import Command from "./Command.mjs";

export default abstract class PlaySoundCommand implements Command {
  #msg: Message;

  constructor(msg: Message) {
    this.#msg = msg;
  }

  abstract pickSound(): string;

  run() {
    const sound = this.pickSound();

    const channel = this.#msg.member?.voice.channel;
    // If the user is in a voice channel, play it!
    if (channel !== null && channel !== undefined) {
      void playSound(channel, sound);
      return;
    }

    // fall back on the text channel
    const messageChannel = this.#msg.channel;
    if (messageChannel.type !== ChannelType.GuildText) {
      console.error("Couldn't play sound, no channel");
      return;
    }

    const attachment = new AttachmentBuilder(sound);
    void messageChannel.send({ files: [attachment] }).catch();
  }
}
