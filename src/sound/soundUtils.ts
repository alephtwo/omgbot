import { globSync } from "glob";
import * as path from "path";
import sample from "../random/sample";

export const SOUND_DIR = path.join(__dirname, "..", "..", "sounds");

export function pickRandomSound(): string {
  const sounds = globFiles(path.join(SOUND_DIR, "**", "*"));
  return sample(sounds);
}

export function pickSound(category: string): string {
  const sounds = globFiles(path.join(SOUND_DIR, category, "**", "*"));
  return sample(sounds);
}

export function getAllCategories(): Set<string> {
  const dirs = globSync(path.join(SOUND_DIR, "*")).map((f) => path.basename(f));
  return new Set(dirs);
}

function globFiles(globstring: string) {
  return globSync(globstring, { nodir: true });
}
