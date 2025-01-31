import fs from 'node:fs'

const input = fs
  .readFileSync('04.txt', 'utf8')
  .split('\n')
  .filter(Boolean)
  .map(line => line.split(''))

function rotate45Degrees(matrix: string[][]): string[][] {
  const n = matrix.length
  const result: string[][] = []

  // Initialize the result array with empty strings
  for (let i = 0; i < 2 * n - 1; i++) {
    result.push(new Array(n).fill(' '))
  }

  for (let row = 0; row < n; row++) {
    for (let col = 0; col < n; col++) {
      result[row + col][Math.floor((n - 1) / 2) + col - row] = matrix[row][col]
    }
  }

  return result
}

function transpose(matrix: string[][]): string[][] {
  const n = matrix.length
  const m = matrix[0].length
  const result: string[][] = []

  for (let col = 0; col < m; col++) {
    result.push([])
    for (let row = 0; row < n; row++) {
      result[col].push(matrix[row][col])
    }
  }

  return result
}

const findDiagonally = (matrix: string[][], i: number, j: number): number => {
  const directions = [
    [0, 1], // right
    [0, -1], // left
    [1, 0], // down
    [-1, 0], // up
    [1, 1], // down-right
    [-1, -1], // up-left
    [1, -1], // down-left
    [-1, 1], // up-right
  ]

  let found = 0
  for (const [dx, dy] of directions) {
    try {
      if (
        matrix[i][j] === 'X' &&
        matrix[i + dx][j + dy] === 'M' &&
        matrix[i + 2 * dx][j + 2 * dy] === 'A' &&
        matrix[i + 3 * dx][j + 3 * dy] === 'S'
      ) {
        found++
      }
    } catch (e) {
      // Ignore out of bounds errors
    }
  }

  return found
}

const countXmas = (matrix: string[][]): number => {
  const str = matrix.map(row => row.join('')).join('\n')
  console.log(str, '\n')
  const matches = str.matchAll(/XMAS/g)
  return [...matches].length
}

let found = 0
let transposed = input

// for (let i = 0; i < 4; i++) {
input.forEach((row, idx) => {
  row.forEach((_, jdx) => {
    found += findDiagonally(transposed, idx, jdx)
  })
})
//   found += countXmas(transposed)
//   transposed = transpose(transposed)
// }

console.log(found)
