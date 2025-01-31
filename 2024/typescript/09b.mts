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

// console.debug(data, free)

const fillArray = (length: number, value: number | null) =>
  new Array(length).fill(value)

const files = data.map((length, index) => new Array(length).fill(index))

const join = (): (number | null)[][] => {
  const freeMapped = free.map(length =>
    length > 0 ? fillArray(length, null) : []
  )

  return files.reduce<(number | null)[][]>(
    (acc: (number | null)[][], file, index) => {
      const f = freeMapped[index] !== undefined ? freeMapped[index] : []
      acc.push(file, f)
      return acc
    },
    []
  )
}

const fileMap = join()
const joinedMap = fileMap.flat()

// console.debug(fileMap, joinedMap)

// console.debug(
//   fileMap.map(a => a.map(a => (a === null ? '.' : a)).join('')).join('')
// )

console.debug(joinedMap.map(a => (a === null ? '.' : a)).join(''))

const getFirstEmptyIndex = (
  array: (number | null)[],
  length: number,
  limit: number
): number => {
  for (let i = 0; i < limit; i++) {
    if (array[i] !== null) {
      continue
    }

    let l = 1

    if (l >= length) {
      return i
    }

    for (let j = i + 1; j < i + length; j++) {
      if (array[j] !== null) {
        break
      }
      l++
      if (l >= length) {
        return i
      }
    }
  }
  return -1
}

for (let i = files.length - 1; i >= 0; i--) {
  const number = files[i][0]
  const fileLength = files[i].length
  const index = joinedMap.indexOf(files[i][0])

  const firstEmptyIndex = getFirstEmptyIndex(joinedMap, fileLength, index)
  console.debug(
    'number',
    number,
    'index',
    index,
    'firstEmptyIndex',
    firstEmptyIndex
  )
  if (firstEmptyIndex !== -1 && firstEmptyIndex <= index) {
    console.debug(
      'firstEmptyIndex',
      firstEmptyIndex,
      'fileLength',
      fileLength,
      'cell',
      number
    )

    for (let j = 0; j < fileLength; j++) {
      joinedMap[firstEmptyIndex + j] = number
      joinedMap[index + j] = null
    }
  }
}

console.debug(joinedMap.map(a => (a === null ? '.' : a)).join(''))

const checkSum = joinedMap.flat().reduce((sum: number, cell, index) => {
  if (cell !== null) {
    return sum + cell * index
  }
  return sum
}, 0)

console.debug(checkSum)
