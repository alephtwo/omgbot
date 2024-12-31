import { Message } from "discord.js";
import Command from "./Command.mjs";

export class InvalidCommand implements Command {
  #message: Message;
  #command: string;

  constructor(message: Message, command: string) {
    this.#message = message;
    this.#command = command;
  }

  run() {
    this.#message.reply(`${this.#command} is not a valid command.`);
    console.error(
      `${this.#message.author.username} issued invalid command: ${this.#command}`,
    );
  }
}
