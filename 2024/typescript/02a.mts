import fs from 'node:fs'

const input = fs.readFileSync('02a.txt', 'utf8')

const inputArray = input.split('\n').map(row => row.split(' ').map(Number))

const isSafe = (array: number[]) => {
  const direction = array[0] - array[1] > 0
  for (let i = 0; i < array.length - 1; i++) {
    if (direction !== array[i] - array[i + 1] > 0) {
      return false
    }

    if (
      Math.abs(array[i] - array[i + 1]) === 0 ||
      Math.abs(array[i] - array[i + 1]) > 3
    ) {
      return false
    }
  }

  return true
}

const res = inputArray.reduce((result, row) => {
  if (isSafe(row)) {
    return ++result
  }

  for (let i = 0; i < row.length; i++) {
    const dampedRow = [...row.slice(0, i), ...row.slice(i + 1)]
    if (isSafe(dampedRow)) {
      return ++result
    }
  }

  return result
}, 0)

console.log(res)
