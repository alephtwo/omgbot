import { assert } from 'chai';
import { parseBangCommand } from '../src/command/parseBangCommand';

describe('parseBangCommand', () => {
  it('No prefix gives null', () => {
    assert.isNull(parseBangCommand('hello world!'));
  });

  it('! prefix gives command', () => {
    assert.equal(parseBangCommand('!omg'), 'omg');
  });
});