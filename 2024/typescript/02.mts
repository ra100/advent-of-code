import fs from 'node:fs'

const input = fs.readFileSync('02.txt', 'utf8')

const inputArray = input.split('\n').map(row => row.split(' ').map(Number))

console.log(inputArray)

const res = inputArray.reduce((result, row) => {
  const direction = row[0] - row[1] > 0
  for (let i = 0; i < row.length - 1; i++) {
    if (direction !== row[i] - row[i + 1] > 0) {
      return result
    }

    if (
      Math.abs(row[i] - row[i + 1]) === 0 ||
      Math.abs(row[i] - row[i + 1]) > 3
    ) {
      return result
    }
  }

  return ++result
}, 0)

console.log(res)
