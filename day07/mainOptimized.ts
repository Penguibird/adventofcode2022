const input = Deno.readTextFileSync('./input.txt').replaceAll('\r', '')

const parseInt = (n: string) => Number.parseInt(n, 10);

const rows = input.split(`
`);


// Solved in O(n*d+m) where n is the number of commands, m is the number of folders and d is the average depth (cca 3 in this case)
const tree = {} as any;
let currentDir = tree; //pointer

const parents = new Map();

const sizesMap = new Map() as Map<object, number>;
const currentArrayParents = [] as object[];

for (const row of rows) {
  console.log(currentArrayParents.length)
  if (row == rows[0] && row == "$ cd /") {
    // Do nothing i guess
    continue;
  }
  if (row == "$ ls") {
    // Do nothing too
    continue;
  }

  const [first, ...rest] = row.split(" ");
  if (first == "dir") {
    const key = rest[0];
    if (!currentDir[key]) {

      currentDir[key] = {};
      parents.set(currentDir[key], currentDir)
    }
  } else if (first == "$") {
    const [cmd, arg] = rest
    if (cmd == "cd") {
      if (arg == "..") {
        currentDir = parents.get(currentDir);
        currentArrayParents.pop();
      } else {
        currentArrayParents.push(currentDir)
        currentDir = currentDir[arg]
      }
    }
  } else {
    const key = rest[0] //+ 'FileUniqueKeyCompletely';
    const size = parseInt(first);
    sizesMap.set(currentDir, (sizesMap.get(currentDir) ?? 0) + size);
    currentArrayParents.forEach(parent => {
      sizesMap.set(parent, (sizesMap.get(parent) ?? 0) + size);
    })
    currentDir[key] = size;
  }
}

// This second recursive looping could definitely be simplified into a single loop

const rootSize = sizesMap.get(tree)!

const sizes = Array.from(sizesMap.entries()).map(([key, val]) => val)
console.log(sizes)

// Sorting unnecessary, could be just a loop keeping track of best Result found

let bestResult = -Infinity;
let bestDif = -Infinity;
for (const size of sizes) {
  const dif = rootSize - size - (70000000 - 30000000)
  // The closer dif is to 0 the better 
  if (dif > bestDif && dif < 0) {
    bestDif = dif
    bestResult = size;
  }
}
console.log(bestResult)
// const a = Object.values(sizes).filter(s => s <= 100000).reduce((a, v) => a + v, 0)
// console.log(Object.values(sizes).length)
// const a = sizes.filter(s => s <= 100000).reduce((a, v) => a + v, 0)
// console.log(a)