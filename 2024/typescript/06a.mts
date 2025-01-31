import fs from 'node:fs'

const input = fs
  .readFileSync('06.txt', 'utf8')
  .split('\n')
  .filter(Boolean)
  .map(row => row.split(''))

const wall = '#'
const empty = '.'
const vertical = '|'
const horizontal = '-'
const obstruction = 'O'
const turn = '+'
const directionsArray = [
  [0, -1],
  [1, 0],
  [0, 1],
  [-1, 0],
]
const directions = {
  '^': 0,
  '>': 1,
  v: 2,
  '<': 3,
}

const path = [vertical, horizontal, turn]
const allPath = [...path, ...Object.values(directions), '.']

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
const initialDirectionIndex = directionIndex
const startingPosition: [number, number] = [position[0], position[1]]

const drawMap = input => input.map(row => row.join('')).join('\n')

const nextDirection = () => (directionIndex + 1) % directionsArray.length

const isInBounds = (x: number, y: number) =>
  x >= 0 && x < input[0].length && y >= 0 && y < input.length

let maxPathLength = 0

while (true) {
  maxPathLength++
  const [x, y] = position
  const [dx, dy] = directionsArray[directionIndex]
  const nx = x + dx
  const ny = y + dy

  if (['.', '-', '|'].includes(input[y][x])) {
    if (dx === 0) {
      if (input[y][x] === '-') {
        input[y][x] = '+'
      } else {
        input[y][x] = '|'
      }
    } else {
      if (input[y][x] === '|') {
        input[y][x] = '+'
      } else {
        input[y][x] = '-'
      }
    }
  }

  if (!isInBounds(nx, ny)) {
    break
  }

  if (input[ny][nx] === wall) {
    input[y][x] = turn
    directionIndex = nextDirection()
    continue
  }

  position = [nx, ny]
}

const obstacles = drawMap(input)
  .split('\n')
  .map(row => row.split(''))

console.log(drawMap(obstacles), '\n')

position = startingPosition
directionIndex = initialDirectionIndex

let loops = 0

const findRepeatingPattern = (path: string[]) => {
  const pathLength = path.length

  if (pathLength < 3) {
    return false
  }

  const last = path.slice(-2).join(';')
  const all = path.join(';')
  const matches = [...all.matchAll(new RegExp(last, 'g'))].length

  if (matches > 1) {
    loops++
    console.log('Loop detected: ', loops)
  }

  return matches > 1
}

console.log('maxPathLength', maxPathLength)

for (let iy = 0; iy < input.length; iy++) {
  for (let jx = 0; jx < input[iy].length; jx++) {
    if ([...path].includes(input[iy][jx])) {
      const path: string[] = []
      let newMap = [...input].map(row => [...row])
      newMap[iy][jx] = obstruction
      let isLooped = false
      let pathLength = 0
      let obstructionHit = 0
      position = startingPosition
      directionIndex = initialDirectionIndex

      while (true) {
        if (obstructionHit > 0) {
          pathLength++
        }
        const [x, y] = position
        if (obstructionHit) {
          path.push(`${x},${y}`)
        }
        const [dx, dy] = directionsArray[directionIndex]
        const nx = x + dx
        const ny = y + dy

        if (pathLength > maxPathLength) {
          isLooped = false
          break
        }

        if (findRepeatingPattern(path)) {
          isLooped = true
          break
        }

        if (!isInBounds(nx, ny)) {
          isLooped = false
          break
        }

        if (newMap[ny][nx] === obstruction) {
          obstructionHit++
          // if (obstructionHit >= 2) {
          //   isLooped = true
          //   break
          // }

          const turnDirection = nextDirection()
          const [tx, ty] = directionsArray[turnDirection]

          // if (![...allPath, '.'].includes(input[y + ty][x + tx])) {
          //   break
          // }
          directionIndex = nextDirection()
          continue
        }

        if (newMap[ny][nx] === wall) {
          directionIndex = nextDirection()
          continue
        }

        position = [nx, ny]
      }

      if (isLooped) {
        obstacles[iy][jx] = obstruction
        // console.log(drawMap(newMap), '\n')
      }
    }
  }
}

// while (true) {
//   const [x, y] = position
//   const [dx, dy] = directionsArray[directionIndex]
//   const nx = x + dx
//   const ny = y + dy

//   if (!isInBounds(nx, ny)) {
//     break
//   }

//   if (obstacles[ny][nx] === wall) {
//     directionIndex = nextDirection()
//     continue
//   }

//   if (
//     Object.keys(directions).includes(obstacles[ny][nx]) ||
//     obstacles[y][x] === turn
//   ) {
//     position = [nx, ny]
//     continue
//   }

//   const turnDirection = nextDirection()
//   const [tx, ty] = directionsArray[turnDirection]

//   if (!isInBounds(x + tx, y + ty)) {
//     position = [nx, ny]
//     continue
//   }

//   if (path.includes(input[y + ty][x + tx])) {
//     obstacles[ny][nx] = obstruction
//   }

//   position = [nx, ny]
// }

const map = drawMap(obstacles)
console.log(map)

const matches = map.matchAll(/O/g)

const result = [...matches].length

console.log(result)
