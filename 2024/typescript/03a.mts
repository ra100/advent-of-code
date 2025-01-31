import fs from 'node:fs'

const input = fs.readFileSync('03.txt', 'utf8')

const inputArray = input.split('\n').filter(Boolean)
console.log(inputArray)

const regex = /(mul\((\d{1,3},\d{1,3})\))|(do\(\))|(don't\(\))/g

const off = "don't()"
const on = 'do()'

const matches = input.matchAll(regex)
let sum = 0
let enabled = true
for (const match of matches) {
  console.log(match)
  if (match[0] === off) {
    enabled = false
    continue
  }

  if (match[0] === on) {
    enabled = true
    continue
  }

  if (enabled) {
    const [, , nums] = match
    const [a, b] = nums.split(',').map(Number)
    sum += a * b
  }
}

console.log(sum)
