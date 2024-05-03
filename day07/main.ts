const input = Deno.readTextFileSync('./input.txt').replaceAll('\r', '')

const parseInt = (n: string) => Number.parseInt(n, 10);

const rows = input.split(`
`);


// type X = string | Record<string, X>;
const tree = {} as any;
let currentDir = tree; //pointer

const parents = new Map();

for (const row of rows) {
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
      } else {
        currentDir = currentDir[arg]
      }
    }
  } else {
    const key = rest[0] //+ 'FileUniqueKeyCompletely';
    const size = parseInt(first);
    currentDir[key] = size;
  }
}

// This second recursive looping could definitely be simplified into a single loop

const sizes = [] as number[]
function getSize(tree: object) {
  // console.log("Getting size of ", tree)
  let size = 0
  Object.entries(tree).forEach(([key, val]) => {
    console.log(key, val)
    if (typeof val == "object") {
      // sizes[key] = getSize(val)
      const x = getSize(val)
      sizes.push(x)
      size += x
    }
    else if (typeof val == "number") {
      size += val;
    } else {
    }
  })
  return size;
}
const rootSize = getSize(tree)

// console.log(sizes)

// Sorting unnecessary, could be just a loop keeping track of best Result found
const sortedSizes = sizes.sort((x, y) => x - y)
for (const size of sortedSizes) {
  if (rootSize - size < (70000000 - 30000000)) {
    console.log(size)
    break
  }
}
// const a = Object.values(sizes).filter(s => s <= 100000).reduce((a, v) => a + v, 0)
// console.log(Object.values(sizes).length)
// const a = sizes.filter(s => s <= 100000).reduce((a, v) => a + v, 0)
// console.log(a)