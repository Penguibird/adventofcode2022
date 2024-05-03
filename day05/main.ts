
const input2 = Deno.readTextFileSync('./input.txt').replaceAll('\r', '')

const parseInt = (n: string) => Number.parseInt(n, 10);

const input = `move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2 `;

const stacks = [
  // "NZ",
  // "DCM",
  // "P"
  "GBDCPR",
  "GVH",
  "MPJDQSN",
  "MNCDGLSP",
  "SLFPCNBJ",
  "STGVZDBQ",
  "QTFHMZB",
  "FBDMC",
  "GQCF"
].map(s => s.split(""));

const timeout = n => new Promise((res) => setTimeout(res, n))
const rows = input2.split(`
`)

for await (const row of rows) {
  const [_, count, ___, source, __, dest] = row.split(' ').map(parseInt);
  const items = stacks[source - 1].splice(0, count)

  stacks[dest - 1] = [...items, ...stacks[dest - 1]];



  // Visualizations added after solved
  await timeout(800);
  console.clear()
  console.log(`
    ${row}
    Moving: ${items.map(c => `[${c}]`).join(' ')}
    `);
  const l = Math.max(...stacks.map(stack => stack.length))
  let out = "  1   2   3   4   5   6   7   8   9 "
  for (let i = 0; i < l; i++) {
    out = ` ${stacks.map(s => i < s.length ? s[i] : null).map(char => char ? `[${char}]` : "   ").join(" ")}
${out}`
  }
  console.log(out)
}



const res = stacks.map(row => row[0]).join("")
console.log(`
`)
console.log("Result: ", res)
console.log(`
`)
