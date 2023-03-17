// Return a sample from the given array.
export default function sample<T>(array: Array<T>): T {
  const index = Math.floor(Math.random() * array.length);
  return array[index];
}
