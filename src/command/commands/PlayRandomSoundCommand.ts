import { pickRandomSound } from '../../sound/soundUtils';
import PlaySoundCommand from './PlaySoundCommand';

export default class PlayRandomSoundCommand extends PlaySoundCommand {
  async pickSound() {
    return await pickRandomSound();
  }
}
