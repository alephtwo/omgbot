// TODO: don't return null here. instead return an object that
// knows what it needs to do.
export function parseBangCommand(msg: string): string | null {
  // If this isn't a command, we can stop.
  if (!msg.startsWith('!')) {
    return null;
  }

  const command = msg.split(' ')[0];

  // Make sure it's a real category
  return command.replace(/^!/, '');
}
