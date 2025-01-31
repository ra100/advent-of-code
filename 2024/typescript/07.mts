import fs from 'node:fs'

const input: [number, number[]][] = fs
  .readFileSync('07.txt', 'utf8')
  .split('\n')
  .filter(Boolean)
  .map(row => row.split(': '))
  .map(([a, b]) => [Number(a), b.split(' ').map(Number)])

console.log(input)

type Operator = (a: number, b: number) => number

const add = (a: number, b: number): number => a + b

const multiply = (a: number, b: number): number => a * b

const or = (a: number, b: number): number => Number(`${a}${b}`)

const operators = [add, multiply, or]

function generateOperatorCombinations(length: number): Operator[][] {
  const result: Operator[][] = []
  const generate = (current: Operator[]) => {
    if (current.length === length) {
      result.push([...current])
      return
    }
    for (const operator of operators) {
      current.push(operator)
      generate(current)
      current.pop()
    }
  }
  generate([])
  return result
}

const isSolvable = (
  result: number,
  operands: number[],
  operator: Operator
): number => {
  const operators = generateOperatorCombinations(operands.length - 1)

  const evaluate = (operands: number[], operators: Operator[]): number => {
    let acc = operands[0]
    for (let i = 0; i < operators.length; i++) {
      acc = operators[i](acc, operands[i + 1])
    }
    return acc
  }

  for (const combination of operators) {
    const nextResult = evaluate(operands, combination)
    if (nextResult === result) return result
  }

  return 0
}

const solvables: number[] = []

const output = input.reduce((acc, [result, operands]) => {
  const res = isSolvable(result, operands, add)
  if (res) solvables.push(result)
  return acc + res
}, 0)

console.log(output)
console.debug(solvables.join(','))
