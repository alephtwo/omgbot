import * as glob from 'glob';
import * as path from 'path';

const root = path.join(__dirname, '..', '..', 'sounds');

export function pickRandomSound(): string {
  const sounds = glob.sync(path.join(root, '**', '*'));
  const index = Math.floor(Math.random() * sounds.length);
  return sounds[index];
}

export function pickSound(category: string): string {
  const sounds = glob.sync(path.join(root, category, '**', '*'));
  const index = Math.floor(Math.random() * sounds.length);
  return sounds[index];
}

export function getAllCategories(): Set<string> {
  const dirs = glob.sync(path.join(root, '*')).map((f) => path.basename(f));
  return new Set(dirs);
}
