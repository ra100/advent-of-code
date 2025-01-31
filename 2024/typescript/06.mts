import fs from 'node:fs'

const input = fs
  .readFileSync('06.txt', 'utf8')
  .split('\n')
  .filter(Boolean)
  .map(row => row.split(''))

const wall = '#'
const empty = '.'
const visited = 'X'
const directionsArray = [
  [0, -1],
  [1, 0],
  [0, 1],
  [-1, 0],
]
const directions = {
  '^': 0,
  '>': 1,
  '<': 2,
  v: 3,
}

const getPosition = (
  matrix: string[][]
): {position: [number, number]; directionIndex: number} => {
  for (let i = 0; i < matrix.length; i++) {
    for (let j = 0; j < matrix[i].length; j++) {
      if (
        matrix[i][j] === '^' ||
        matrix[i][j] === '>' ||
        matrix[i][j] === '<' ||
        matrix[i][j] === 'v'
      ) {
        return {
          position: [j, i],
          directionIndex: directions[matrix[i][j]],
        }
      }
    }
  }

  throw new Error('No starting position found')
}

let {position, directionIndex} = getPosition(input)

const drawMap = () => input.map(row => row.join('')).join('\n')

const nextDirection = () => (directionIndex + 1) % directionsArray.length

while (true) {
  console.debug(drawMap(), '\n')
  const [x, y] = position
  const [dx, dy] = directionsArray[directionIndex]
  const nx = x + dx
  const ny = y + dy
  input[y][x] = visited

  if (ny < 0 || nx < 0 || ny >= input.length || nx >= input[0].length) {
    break
  }

  if (input[ny][nx] === wall) {
    directionIndex = nextDirection()
  } else {
    position = [nx, ny]
  }
}

const map = drawMap()

const matches = map.matchAll(/X/g)

const result = [...matches].length

console.log(result)
