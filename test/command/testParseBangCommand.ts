import { assert } from 'chai';
import { parseBangCommand } from '../../src/command/parseBangCommand';

describe('parseBangCommand', () => {
  it('No prefix gives null', () => {
    assert.isNull(parseBangCommand('hello world!'));
  });

  it('! prefix gives command', () => {
    assert.equal(parseBangCommand('!omg'), 'omg');
  });

  it('lots of typing gives first command', () => {
    assert.equal(parseBangCommand('!omg !a !b !c'), 'omg');
  });

  it('command can be present within string', () => {
    assert.equal(parseBangCommand('you should add !omg'), 'omg');
  });

  it('! by itself parses', () => {
    assert.equal(parseBangCommand('!'), '');
  });
});
