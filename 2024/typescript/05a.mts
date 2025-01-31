import fs from 'node:fs'

const input = fs.readFileSync('05.txt', 'utf8').split('\n\n').filter(Boolean)

const rules = input[0]
  .split('\n')
  .map(line => line.split('|').map(Number))
  .reduce((acc, rule) => {
    if (acc.has(rule[0])) {
      acc.get(rule[0])?.push(rule[1])
      return acc
    }
    acc.set(rule[0], [rule[1]])
    return acc
  }, new Map<number, number[]>())
const manuals = input[1]
  .split('\n')
  .filter(Boolean)
  .map(line => line.split(',').map(Number))

console.debug({rules, manuals})

const wrong = manuals.filter(manual => {
  return !manual.every((num, index) => {
    const rule = rules.get(num)
    if (!rule) {
      return true
    }

    return rule.every(part => {
      const idx = manual.findIndex(num => num === part)

      if (idx === -1) {
        return true
      }

      if (idx > index) {
        return true
      }

      return false
    })
  })
})
console.log(wrong)

const sorted = [...wrong].map(manual =>
  manual.sort((a, b) => {
    const ruleA = rules.get(a)
    const ruleB = rules.get(b)
    const idxA = manual.indexOf(a)
    const idxB = manual.indexOf(b)

    const beforeA = ruleA?.includes(b)
    const beforeB = ruleB?.includes(a)

    if (!beforeA && !beforeB) {
      return idxA - idxB
    }

    if (beforeA && !beforeB) {
      return -1
    }

    if (!beforeA && beforeB) {
      return 1
    }

    return idxA - idxB
  })
)

const sum = sorted
  .map(manual => manual[(manual.length - 1) / 2])
  .reduce((acc, num) => acc + num, 0)

console.log(sorted)
console.log(sum)
