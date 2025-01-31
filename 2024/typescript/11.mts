import fs from 'fs'

const ITERATIONS = 75

function main() {
  const input: number[] = fs
    .readFileSync('11.txt', 'utf8')
    .split('\n')
    .filter(Boolean)
    .flatMap(line => line.split(' ').map(Number))

  console.log(input)

  const start = Date.now()
  const cache: Map<string, number> = new Map()
  const res = applyRulesRecursive(input, ITERATIONS, cache)
  const duration = Date.now() - start

  const result = res.reduce((sum, count) => sum + count, 0)

  console.log(result)
  console.log(`Time taken: ${duration}ms`)
  console.log(cache.size)
}

function getValueByRule(num: number): number[] {
  if (num === 0) {
    return [1]
  }

  const numStr = num.toString()
  if (numStr.length % 2 === 0) {
    const div = Math.pow(10, numStr.length / 2)
    return [Math.floor(num / div), num % div]
  }

  const result = num * 2024
  if (result > Number.MAX_SAFE_INTEGER) {
    console.log(`Overflow detected for num: ${num}`)
    return [Number.MAX_SAFE_INTEGER]
  }

  return [result]
}

function applyRulesRecursive(
  input: number[],
  iteration: number,
  cache: Map<string, number>
): number[] {
  if (iteration === 0) {
    return input.map(() => 1)
  }

  const next: number[] = []
  for (const num of input) {
    const cacheKey = `${num},${iteration}`
    if (cache.has(cacheKey)) {
      next.push(cache.get(cacheKey)!)
    } else {
      const nextSubArray = getValueByRule(num)
      const result = applyRulesRecursive(nextSubArray, iteration - 1, cache)
      const count = result.reduce((sum, count) => sum + count, 0)
      cache.set(cacheKey, count)
      next.push(count)
    }
  }

  return next
}

main()
