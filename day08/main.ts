const input = Deno.readTextFileSync('./input2.txt').replaceAll('\r', '')

const parseInt = (n: string) => Number.parseInt(n, 10);

const rows = input.split(`
`).map(r => r.split('').map(parseInt));


let visibleTrees = rows.map(r => r.map(t => 0));





// Top to bottom
// Left to right
for (let i = 0; i < rows.length; i += 1) {
  const row = rows[i];
  let highestPoint = 0;
  for (let j = 0; j < row.length; j++) {
    // console.log(i,j)
    const tree = row[j];
    // console.log(i, j, tree, highestPoint)
    if (tree > highestPoint || (i == 0 || j == 0 || j == rows[0].length - 1 || i == rows.length - 1)) {
      // console.log("Mark")
      highestPoint = tree;
      visibleTrees[i][j] = 1;
    }
  }
}

// Top to bottom 
// Right to left
for (let i = 0; i < rows.length; i += 1) {
  const row = rows[i];
  let highestPoint = 0;
  for (let j = row.length - 1; j >= 0; j--) {
    // console.log(i,j)
    const tree = row[j];
    // console.log(i, j, tree, highestPoint)
    if (tree > highestPoint || (i == 0 || j == 0 || j == rows[0].length - 1 || i == rows.length - 1)) {
      // console.log("Mark")
      highestPoint = tree;
      visibleTrees[i][j] = 1;
    }
  }
}

// Left to right
// Top to bottom
for (let j = 0; j < rows[0].length; j++) {
  let highestPoint = 0;
  for (let i = 0; i < rows.length; i += 1) {
    // console.log(i,j)
    const tree = rows[i][j];
    // console.log(i, j, tree, highestPoint)
    if (tree > highestPoint || (i == 0 || j == 0 || j == rows[0].length - 1 || i == rows.length - 1)) {
      // console.log("Mark")
      highestPoint = tree;
      visibleTrees[i][j] = 1;
    }
  }
}



// Left to right
// Bottom to top
for (let j = 0; j < rows[0].length; j++) {
  let highestPoint = 0;
  for (let i = rows.length - 1; i >= 0; i--) {
    // console.log(i,j)
    const tree = rows[i][j];
    // console.log(i, j, tree, highestPoint)
    if (tree > highestPoint || i == 0 || j == 0 || j == rows[0].length - 1 || i == rows.length - 1) {
      // console.log("Mark")
      highestPoint = tree;
      visibleTrees[i][j] = 1;
    }
  }
}



// console.log(rows.map(r => r.join('')).join(`
// `))
// console.log("---")
// console.log(visibleTrees.flat().reduce((x, y) => x + y, 0))
// 


const r = rows.map((row, rowI) => row.map((tree, treeI) => {
  if (visibleTrees[rowI][treeI] == 0)
    return 0;
  if (rowI == 0 || rowI == rows.length - 1 || treeI == 0 || treeI == row.length)
    return 0;

  let total = 1;
  let score = 0;

  score = 0;
  for (let i = treeI + 1; i < row.length; i++) {
    if (row[i] < tree) 
    score++
    else {
      score++
      break
    }
  }
  total *= score;
  console.log(rowI, treeI, score)

  score = 0;
  for (let i = treeI - 1; i >= 0; i--) {
    if (row[i] < tree) score++
    else {
      score++
      break
    } 
  }
  total *= score;
  console.log(rowI, treeI, score)

  score = 0;
  for (let i = rowI + 1; i < rows.length; i++) {
    if (rows[i][treeI] < tree) score++
    else {
      score++
      break
    };
  }
  console.log(rowI, treeI, score)

  total *= score;

  score = 0;
  for (let i = rowI - 1; i >= 0; i--) {
    if (rows[i][treeI] < tree) score++
    else {
      score++
      break
    };
  }
  console.log(rowI, treeI, score)


  total *= score;
  return total;
}))

console.log(r.map(r => r.join(' ')).join(`
`))
console.log(Math.max(...r.flat()))