import * as glob from 'glob';
import * as path from 'path';

const root = path.join(__dirname, '..', 'sounds');

export function pickSound (category: string) {
    const sounds = getSounds(category);
    const index = Math.floor(Math.random() * sounds.length);
    return sounds[index];
}

function getSounds (category: string) {
    return glob.sync(path.join(root, category, '**', '*'));
}