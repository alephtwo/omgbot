import { Message } from "discord.js";
import { parseBangCommand } from "./parseBangCommand.mjs";
import { describe, it, expect } from "vitest";
import { mock, when } from "strong-mock";
import PlayRandomSoundCommand from "./commands/PlayRandomSoundCommand.mjs";
import PlaySoundFromCategoryCommand from "./commands/PlaySoundFromCategoryCommand.mjs";

describe("parseBangCommand", () => {
  it("No prefix gives null", () => {
    const { parsed, command } = parseBangCommand(mockMessage("hello world!"));
    expect(parsed).toBeUndefined();
    expect(command).toBeUndefined();
  });

  it("! prefix gives command", () => {
    const { parsed, command } = parseBangCommand(mockMessage("!omg"));
    expect(parsed).toEqual("omg");
    expect(command).toBeInstanceOf(PlaySoundFromCategoryCommand);
  });

  it("lots of typing gives first command", () => {
    const { parsed, command } = parseBangCommand(mockMessage("!omg !a !b !c"));
    expect(parsed).toEqual("omg");
    expect(command).toBeInstanceOf(PlaySoundFromCategoryCommand);
  });

  it("command can be present within string", () => {
    const { parsed, command } = parseBangCommand(
      mockMessage("you should add !omg"),
    );
    expect(parsed).toEqual("omg");
    expect(command).toBeInstanceOf(PlaySoundFromCategoryCommand);
  });

  it("! by itself parses", () => {
    const { parsed, command } = parseBangCommand(mockMessage("!"));
    expect(parsed).toEqual("");
    expect(command).toBeInstanceOf(PlayRandomSoundCommand);
  });
});

function mockMessage(s: string): Message {
  const message = mock<Message>();
  when(() => message.content).thenReturn(s);
  return message;
}
