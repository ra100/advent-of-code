import fs from 'node:fs'

type Point = [number, number]

const input: string[][] = fs
  .readFileSync('08.txt', 'utf8')
  .split('\n')
  .filter(Boolean)
  .map(row => row.split(''))

const antennaMap = new Map<string, Point[]>()

for (let row = 0; row < input.length; row++) {
  for (let col = 0; col < input[row].length; col++) {
    const cell = input[row][col]
    if (!cell.match(/[a-zA-Z0-9]/)) {
      continue
    }
    if (!antennaMap.has(cell)) {
      antennaMap.set(cell, [])
    }
    antennaMap.get(cell)!.push([col, row])
  }
}

console.debug(antennaMap)

const bounds = [input[0].length, input.length]

const blockers: Point[] = []

const isInBounds = (point: Point) => {
  const [x, y] = point
  return x >= 0 && y >= 0 && x < bounds[0] && y < bounds[1]
}

const addBlocker = (pointA: Point, pointB: Point) => {
  const [x1, y1] = pointA
  const [x2, y2] = pointB

  const dx = x1 - x2
  const dy = y1 - y2

  let blockerA: Point = [x1 + dx, y1 + dy]

  while (isInBounds(blockerA)) {
    blockers.push(blockerA)
    blockerA = [blockerA[0] + dx, blockerA[1] + dy]
  }

  let blockerB: Point = [x2 - dx, y2 - dy]

  while (isInBounds(blockerB)) {
    blockers.push(blockerB)
    blockerB = [blockerB[0] - dx, blockerB[1] - dy]
  }
}

const findBlockers = (coordinates: Point[]) => {
  const [first, ...rest] = coordinates
  rest.forEach(point => addBlocker(first, point))

  if (rest.length <= 1) {
    return
  }

  findBlockers(rest)
}

let selfBlockers = 0

antennaMap.forEach((coordinates, antenna) => {
  if (coordinates.length <= 1) {
    antennaMap.delete(antenna)
    return
  }

  selfBlockers += coordinates.length

  findBlockers(coordinates)
})

console.log(blockers)

blockers.forEach(blocker => {
  const [x, y] = blocker

  if (x < 0 || y < 0 || x >= input.length || y >= input[0].length) {
    return
  }

  if (input[y][x] === '.') {
    input[y][x] = '#'
  }
})

const outputMap = input.map(a => a.join('')).join('\n')

console.log(outputMap)

const result = [...outputMap.matchAll(/#/g)].length

console.log(result + selfBlockers)
