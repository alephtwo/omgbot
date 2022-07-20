export default interface Command {
  run: () => Promise<void>;
}
