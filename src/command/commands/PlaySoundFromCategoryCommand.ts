import { Message } from "discord.js";
import { pickSound } from "../../sound/soundUtils";
import PlaySoundCommand from "./PlaySoundCommand";

export default class PlaySoundFromCategoryCommand extends PlaySoundCommand {
  #category: string;

  constructor(msg: Message, category: string) {
    super(msg);
    this.#category = category;
  }

  pickSound() {
    return pickSound(this.#category);
  }
}
