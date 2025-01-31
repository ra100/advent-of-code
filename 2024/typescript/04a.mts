import fs from 'node:fs'

const input = fs
  .readFileSync('04.txt', 'utf8')
  .split('\n')
  .filter(Boolean)
  .map(line => line.split(''))

function rotate90Degrees(matrix: string[][]): string[][] {
  const n = matrix.length
  const m = matrix[0].length
  const result: string[][] = []

  for (let col = 0; col < m; col++) {
    result.push([])
    for (let row = n - 1; row >= 0; row--) {
      result[col].push(matrix[row][col])
    }
  }

  return result
}

const findDiagonally = (matrix: string[][], i: number, j: number): number => {
  const directions = [
    // [0, 1], // right
    // [0, -1], // left
    // [1, 0], // down
    // [-1, 0], // up
    [1, 1], // down-right
    // [-1, -1], // up-left
    // [1, -1], // down-left
    // [-1, 1], // up-right
  ]

  let f = 0
  for (const [dx, dy] of directions) {
    try {
      if (
        matrix[i][j] === 'A' &&
        matrix[i - 1][j - 1] === 'M' &&
        matrix[i + 1][j + 1] === 'S' &&
        matrix[i - 1][j + 1] === 'M' &&
        matrix[i + 1][j - 1] === 'S'
      ) {
        f++
      }
    } catch (e) {
      // Ignore out of bounds errors
    }
  }

  return f
}

let found = 0
let transposed = input

for (let i = 0; i < 4; i++) {
  transposed.forEach((row, idx) => {
    row.forEach((_, jdx) => {
      found += findDiagonally(transposed, idx, jdx)
    })
  })
  transposed = rotate90Degrees(transposed)
}

console.log(found)
