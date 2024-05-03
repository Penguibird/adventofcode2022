const input2 = Deno.readTextFileSync('./input.txt').replaceAll('\r', '')

const parseInt = (n: string) => Number.parseInt(n, 10);

const input = `bvwbjplbgvbhsrlpgdmjqwftvncz`;


const chars = input.split("")

for (let i = 0; i < chars.length - 4; i++) {
  const elements = chars.slice(i, i + 4);
  console.log(elements)
  if (elements.reduce((a, v) => {
    if (a.includes(v))
      return a;
    else
      return [...a, v]
  }, [] as string[]).length == 4) {
    console.log(i + 4)
    break;
  }
}