import * as glob from 'glob';
import * as path from 'path';

const root = path.join(__dirname, '..', 'sounds');

export function pickSound (category: string) {
    const sounds = getSounds(category);
    const index = Math.floor(Math.random() * sounds.length);
    return sounds[index];
}

export function getAllCategories (): Set<String> {
    const dirs = glob.sync(path.join(root, '*'))
        .map(f => path.basename(f));
    return new Set(dirs);
}

function getSounds (category: string) {
    return glob.sync(path.join(root, category, '**', '*'));
}