import { globSync } from "glob";
import * as path from "path";
import sample from "../random/sample.mjs";

export function getSoundDir(): string {
  const env = process.env["OMGBOT_SOUND_DIR"];
  if (env !== undefined) {
    return env;
  }
  return path.join(import.meta.dirname, "..", "sounds");
}

export function pickRandomSound(): string {
  const sounds = globFiles(path.join(getSoundDir(), "**", "*"));
  return sample(sounds);
}

export function pickSound(category: string): string {
  const sounds = globFiles(path.join(getSoundDir(), category, "**", "*"));
  return sample(sounds);
}

export function getAllCategories(): Set<string> {
  const dirs = globSync(path.join(getSoundDir(), "*")).map((f) =>
    path.basename(f),
  );
  return new Set(dirs);
}

function globFiles(globstring: string) {
  return globSync(globstring, { nodir: true });
}
