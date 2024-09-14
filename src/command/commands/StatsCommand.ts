import { ChannelType, Message, bold } from "discord.js";
import Command from "./Command";
import { globSync } from "glob";
import { SOUND_DIR } from "../../sound/soundUtils";
import * as path from "path";

export default class StatsCommand implements Command {
  #msg: Message;

  constructor(msg: Message) {
    this.#msg = msg;
  }

  run() {
    const dirs = globSync(path.join(SOUND_DIR, "*"));
    const counts = dirs.map((dir) => ({
      sound: path.basename(dir),
      count: this.countSounds(dir),
    }));

    const channel = this.#msg.channel;
    if (channel.type !== ChannelType.GuildText) {
      console.error("Couldn't send stats, no channel");
      return;
    }

    channel.send({ content: this.buildMessage(counts) }).catch(console.error);
  }

  private countSounds(dir: string): number {
    return globSync(path.join(dir, "*.mp3")).length;
  }

  private buildMessage(counts: Array<{ sound: string; count: number }>) {
    const total = counts.map((a) => a.count).reduce((a, b) => a + b);
    const top3 = counts
      .toSorted((a, b) => b.count - a.count)
      .slice(0, 3)
      .map((s) => `${s.sound} (${s.count})`);

    const lines = [
      `There are ${bold(total.toString())} total sounds across ${bold(counts.length.toString())} commands.`,
      bold("Top 3:"),
      `:first_place: ${top3[0]}`,
      `:second_place: ${top3[1]}`,
      `:third_place: ${top3[2]}`,
    ];
    return lines.join("\n");
  }
}
