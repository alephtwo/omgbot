import { Message, unorderedList } from "discord.js";
import { getAllCategories } from "../../sound/soundUtils.mjs";
import Command from "./Command.mjs";

const categories = getAllCategories();

export default class HelpCommand implements Command {
  #msg: Message;

  constructor(msg: Message) {
    this.#msg = msg;
  }

  run() {
    const help = Array.from(categories)
      .sort((a: string, b: string) => a.localeCompare(b))
      .map((c) => `\`!${c}\``);

    void this.#msg.author.send(unorderedList(help)).catch();
  }
}
