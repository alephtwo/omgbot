import { expect } from 'chai';
import { glob } from 'glob';
import * as path from 'path';
import { getAllCategories } from '../../src/sound/soundUtils';

describe('Sound Utils', () => {
  it('Get all categories returns everything', () => {
    const dir = path.join(__dirname, '..', '..', 'sounds', '*');
    const expected = new Set(glob.sync(dir).map((e) => path.basename(e)));

    const sounds = getAllCategories();

    expect(sounds).to.have.all.keys(...expected);
  });
});
