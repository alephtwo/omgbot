import { Message } from "discord.js";
import { parseBangCommand } from "../../src/command/parseBangCommand";
import { mock, when } from "strong-mock";
import PlayRandomSoundCommand from "../../src/command/commands/PlayRandomSoundCommand";
import { assert } from "chai";
import PlaySoundFromCategoryCommand from "../../src/command/commands/PlaySoundFromCategoryCommand";

describe("parseBangCommand", () => {
  it("No prefix gives null", () => {
    const { parsed, command } = parseBangCommand(mockMessage("hello world!"));
    assert.isUndefined(parsed);
    assert.isUndefined(command);
  });

  it("! prefix gives command", () => {
    const { parsed, command } = parseBangCommand(mockMessage("!omg"));
    assert.equal(parsed, "omg");
    assert.instanceOf(command, PlaySoundFromCategoryCommand);
  });

  it("lots of typing gives first command", () => {
    const { parsed, command } = parseBangCommand(mockMessage("!omg !a !b !c"));
    assert.equal(parsed, "omg");
    assert.instanceOf(command, PlaySoundFromCategoryCommand);
  });

  it("command can be present within string", () => {
    const { parsed, command } = parseBangCommand(mockMessage("you should add !omg"));
    assert.equal(parsed, "omg");
    assert.instanceOf(command, PlaySoundFromCategoryCommand);
  });

  it("! by itself parses", () => {
    const { parsed, command } = parseBangCommand(mockMessage("!"));
    assert.equal(parsed, "");
    assert.instanceOf(command, PlayRandomSoundCommand);
  });
});

function mockMessage(s: string): Message {
  const message = mock<Message>();
  when(() => message.content).thenReturn(s);
  return message;
}
