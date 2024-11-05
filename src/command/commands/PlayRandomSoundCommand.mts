import { pickRandomSound } from "../../sound/soundUtils.mjs";
import PlaySoundCommand from "./PlaySoundCommand.mjs";

export default class PlayRandomSoundCommand extends PlaySoundCommand {
  pickSound() {
    return pickRandomSound();
  }
}
