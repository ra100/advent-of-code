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

console.debug(data.length, free.length)

const fillArray = (length: number, value: number | null) =>
  new Array(length).fill(value)

const join = (a: number[], b: number[]): (number | null)[] => {
  const files = a.map((length, index) => new Array(length).fill(index))
  const free = b.map(length => (length > 0 ? fillArray(length, null) : []))

  return files.reduce<number[]>((acc, file, index) => {
    const f = free[index] !== undefined ? free[index] : []
    return acc.concat(file).concat(f)
  }, [])
}

// const filesToTake = data
//   .map((length, index) => new Array(length).fill(`${index}`))
//   .join('')
//   .split('')

const fileMap = join(data, free)

// console.debug(fileMap.join(';'))

let fileIndex = fileMap.length - 1

for (let freeSpaceIndex = 0; freeSpaceIndex < fileIndex; freeSpaceIndex++) {
  const cell = fileMap[freeSpaceIndex]
  if (cell !== null) {
    continue
  }
  while (fileMap[fileIndex] === null) {
    fileIndex--
  }
  if (fileIndex <= freeSpaceIndex) {
    break
  }
  fileMap[freeSpaceIndex] = fileMap[fileIndex]
  fileMap[fileIndex] = null
  fileIndex--
}

console.debug(fileMap.map(a => (a === null ? '.' : a)).join(''))

const checkSum = fileMap.reduce((sum: number, cell, index) => {
  if (cell !== null) {
    return sum + cell * index
  }
  return sum
}, 0)

console.debug(checkSum)
