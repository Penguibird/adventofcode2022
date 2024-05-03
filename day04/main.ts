
const input2 = Deno.readTextFileSync('./input.txt').replaceAll('\r', '')

const parseInt = n => Number.parseInt(n, 10);

const input = `2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8`

const rows = input2.split(`
`)
  .map(row => row.split(',')
    .map(range => {
      return range.split('-');
    })
    .flat()
    .map(parseInt)
  ).reduce((a, [startA, endA, startB, endB]) => {
    console.log(startA, endA, startB, endB)
    if (
      (startB >= startA && startB <= endA)
      || (endB >= startA && endB <= endA)
      || (startA <= startB && endA >= endB)
      || (startB <= startA && endB >= endA)
    ) {
      console.log(true)
      return ++a;
    }
    return a;
  }, 0);





console.log(rows)