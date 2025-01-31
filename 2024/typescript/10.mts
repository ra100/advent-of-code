import fs from 'node:fs'

const input: number[][] = fs
  .readFileSync('10.txt', 'utf8')
  .split('\n')
  .filter(Boolean)
  .map(row => row.split('').map(Number))

console.log(input)

const directions = [
  [0, 1], // right
  [1, 0], // down
  [0, -1], // left
  [-1, 0], // up
]

function isValid(x: number, y: number, grid: number[][]): boolean {
  return x >= 0 && x < grid.length && y >= 0 && y < grid[0].length
}

function bfs(startX: number, startY: number, grid: number[][]): number {
  const queue: [number, number, number][] = [[startX, startY, 0]]
  let count = 0

  while (queue.length > 0) {
    const [x, y, value] = queue.shift()!

    if (grid[x][y] === 9) {
      count++
      continue
    }

    for (const [dx, dy] of directions) {
      const newX = x + dx
      const newY = y + dy

      if (isValid(newX, newY, grid) && grid[newX][newY] === value + 1) {
        queue.push([newX, newY, grid[newX][newY]])
      }
    }
  }

  return count
}

let totalCount = 0

for (let i = 0; i < input.length; i++) {
  for (let j = 0; j < input[i].length; j++) {
    if (input[i][j] === 0) {
      totalCount += bfs(i, j, input)
    }
  }
}

console.log(totalCount)
