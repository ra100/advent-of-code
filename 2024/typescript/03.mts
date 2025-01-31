import fs from 'node:fs'

const input = fs.readFileSync('03.txt', 'utf8')

const inputArray = input.split('\n').filter(Boolean)
console.log(inputArray)

const regex = /mul\((\d{1,3},\d{1,3})\)/g

const result = inputArray
  .map(line => {
    const matches = line.matchAll(regex)
    let sum = 0
    for (const match of matches) {
      const [, nums] = match
      const [a, b] = nums.split(',').map(Number)
      sum += a * b
    }
    return sum
  })
  .reduce((acc, curr) => acc + curr, 0)

console.log(result)
