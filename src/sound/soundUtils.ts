import * as glob from 'glob';
import * as path from 'path';
import sample from '../random/sample';

const root = path.join(__dirname, '..', '..', 'sounds');

export async function pickRandomSound(): Promise<string> {
  const sounds = globFiles(path.join(root, '**', '*'));
  return sample(sounds);
}

export async function pickSound(category: string): Promise<string> {
  const sounds = globFiles(path.join(root, category, '**', '*'));
  return sample(sounds);
}

export function getAllCategories(): Set<string> {
  const dirs = glob.sync(path.join(root, '*')).map((f) => path.basename(f));
  return new Set(dirs);
}

function globFiles(globstring: string) {
  return glob.sync(globstring, { nodir: true });
}
