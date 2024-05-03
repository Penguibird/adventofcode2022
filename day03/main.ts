
const input2 = Deno.readTextFileSync('./input.txt').replaceAll('\r', '')


const input = `vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw`

const rows = input2.split(`
`)


// const split = rows.map(row => {
//   const i = (row.length / 2);
//   return [
//     row.slice(0, i)
//     ,
//     row.slice(i)
//   ]
// })

// const uniq = rows.map((row) => {
// })
// 
const uniq = [] as string[];
for (let i = 0; i < rows.length ; i += 3) {
  const row = rows[i];
  const row2 = rows[i + 1]
  const row3 = rows[i + 2]

  for (const char of (row.split(''))) {
    if (row2.includes(char) && row3.includes(char)) {
      uniq.push(char);
      break
    }
  }
}

const mapped = uniq.map(letter => {
  if (!letter)
    return;

  let n = 0;
  if (/[A-Z]/.test(letter))
    n = 26 + 26 + 6;

  n += letter?.charCodeAt(0)

  n -= 64 + 26 + 6;
  return n;

})

const sum = mapped.reduce((x, y) => x + y, 0)

console.log(uniq)
console.log(sum)
// console.log(uniq)
// console.log(mapped)