import axios from 'axios';

// Return a sample from the given array.
export default async function sample<T>(array: Array<T>): Promise<T> {
  try {
    return await anuqrng(array);
  } catch (err) {
    console.error('using fallback');
    console.error(err);
    return fallback(array);
  }
}

function fallback<T>(array: Array<T>): T {
  const index = Math.floor(Math.random() * array.length);
  return array[index];
}

async function anuqrng<T>(array: Array<T>): Promise<T> {
  const res = await axios.get('https://qrng.anu.edu.au/API/jsonI.php?length=1&type=uint16', {
    timeout: 2500, // ms
  });

  // This is a number between 0-65535
  // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access
  const n = res.data.data[0] as number;
  // n / 65535 is the % of 65535 - we will use this to get a random
  // within our range
  const random = n / 65535;
  // treat that number just like the output of Math.random()
  const index = Math.floor(random * array.length);
  return array[index];
}
