import fs from 'node:fs'

const input: number[] = fs
  .readFileSync('09.txt', 'utf8')
  .split('\n')
  .filter(Boolean)
  .map(row => row.split('').map(Number))[0]

const data: number[] = []
const free: number[] = []

input.forEach((num, index) => {
  if (index % 2 === 1) {
    free.push(num)
  } else {
    data.push(num)
  }
})

console.debug(data, free)

const fillArray = (length: number, value: number | null) =>
  new Array(length).fill(value)

const join = (a: number[], b: number[]): (number | null)[][] => {
  const files = a.map((length, index) => new Array(length).fill(index))
  const free = b.map(length => (length > 0 ? fillArray(length, null) : [0, []]))

  return files.reduce<(number | null)[][]>(
    (acc: (number | null)[][], file, index) => {
      const f = free[index] !== undefined ? free[index] : []
      acc.push(file, f)
      return acc
    },
    []
  )
}

const fileMap = join(data, free)

console.debug(fileMap)

console.debug(
  fileMap.map(a => a.map(a => (a === null ? '.' : a)).join('')).join('')
)

const getEmptySlots = (cell: (number | null)[]): number =>
  cell.filter(slot => slot === null).length

for (let i = fileMap.length - 1; i >= 0; i--) {
  const cell = fileMap[i]
  if (cell.length === 0 || cell[0] === null) {
    continue
  }
  if (cell[0] === null) {
    continue
  }

  for (let j = 0; j <= i; j++) {
    const targetCell = fileMap[j]
    if (targetCell.length === 0) {
      continue
    }
    const emptySlots = getEmptySlots(targetCell)
    if (emptySlots < cell.length) {
      continue
    }
    const firstEmptySlotIndex = targetCell.indexOf(null)
    fileMap[j].splice(firstEmptySlotIndex, cell.length, ...cell)
    fileMap[i] = fileMap[i].fill(null)
    break
  }
}

console.debug(
  fileMap.map(a => a.map(a => (a === null ? '.' : a)).join('')).join('')
)

const checkSum = fileMap.flat().reduce((sum: number, cell, index) => {
  if (cell !== null) {
    return sum + cell * index
  }
  return sum
}, 0)

console.debug(checkSum)
