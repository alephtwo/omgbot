import Command from './Command';

export class InvalidCommand implements Command {
  #command: string;

  constructor(command: string) {
    this.#command = command;
  }

  run() {
    console.error(`Invalid command: ${this.#command}`);
    return Promise.resolve();
  }
}
