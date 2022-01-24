// TODO: don't return null here. instead return an object that
// knows what it needs to do.
export function parseBangCommand(msg: string): string | null {
  // Find if there is any string that might be a command
  const commands = msg.split(' ').filter((c) => c.startsWith('!'));

  // If there aren't any commands, just bail out
  if (commands.length === 0) {
    return null;
  }

  // Grab the first command, strip the bang
  return commands[0].replace(/^!/, '').trim();
}
