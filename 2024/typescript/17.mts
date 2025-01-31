import {performance} from 'perf_hooks'

function main() {
  let start = performance.now()
  for (let i = 190_000_000; i < 2000000000; i++) {
    if (i % 1_000_000 === 0) {
      let duration = performance.now() - start
      console.log(`${i / 1_000_000}: ${duration} ms`)
      start = performance.now()
    }

    let output: number[] = []

    let registerA = i
    let registerB = 0
    let registerC = 0

    let program = [2, 4, 1, 5, 7, 5, 4, 5, 0, 3, 1, 6, 5, 5, 3, 0]

    let registers = [registerA, registerB, registerC]
    let pointer = 0

    while (pointer < program.length) {
      let opcode = program[pointer]
      let operand = program[pointer + 1]
      let res = false
      switch (opcode) {
        case 0:
          res = adv(registers, operand, pointer)
          break
        case 1:
          res = bxl(registers, operand, pointer)
          break
        case 2:
          res = bst(registers, operand, pointer)
          break
        case 3:
          res = jnz(registers, operand, pointer)
          break
        case 4:
          res = bxc(registers, operand, pointer)
          break
        case 5:
          res = out(registers, operand, pointer, output)
          break
        case 6:
          res = bdv(registers, operand, pointer)
          break
        case 7:
          res = cdv(registers, operand, pointer)
          break
        default:
          console.error(`Invalid opcode: ${opcode}`)
          break
      }

      if (!res) {
        break
      }
    }
  }
}

function getComboOperand(registers: number[], comboOperand: number): number {
  if (comboOperand >= 7) {
    throw new Error(`Invalid combo operand: ${comboOperand}`)
  }
  if (comboOperand <= 3) {
    return comboOperand
  } else {
    return registers[comboOperand - 4]
  }
}

function adv(
  registers: number[],
  comboOperand: number,
  pointer: number
): boolean {
  let a = registers[0]
  let b = getComboOperand(registers, comboOperand)

  registers[0] = Math.floor(a / Math.pow(b, 2))
  pointer += 2
  return true
}

function bxl(
  registers: number[],
  literalOperand: number,
  pointer: number
): boolean {
  let a = registers[1]
  let b = literalOperand

  registers[1] = a ^ b
  pointer += 2
  return true
}

function bst(
  registers: number[],
  literalOperand: number,
  pointer: number
): boolean {
  let a = registers[2]
  let b = literalOperand

  registers[2] = a + b
  pointer += 2
  return true
}

function jnz(
  registers: number[],
  literalOperand: number,
  pointer: number
): boolean {
  let a = registers[0]

  if (a !== 0) {
    pointer += literalOperand
  } else {
    pointer += 2
  }
  return true
}

function bxc(
  registers: number[],
  literalOperand: number,
  pointer: number
): boolean {
  let a = registers[1]
  let b = literalOperand

  registers[1] = a & b
  pointer += 2
  return true
}

function out(
  registers: number[],
  literalOperand: number,
  pointer: number,
  output: number[]
): boolean {
  let a = registers[0]

  output.push(a)
  pointer += 2
  return true
}

function bdv(
  registers: number[],
  literalOperand: number,
  pointer: number
): boolean {
  let a = registers[2]
  let b = literalOperand

  registers[2] = Math.floor(a / b)
  pointer += 2
  return true
}

function cdv(
  registers: number[],
  literalOperand: number,
  pointer: number
): boolean {
  let a = registers[0]
  let b = literalOperand

  registers[0] = Math.floor(a / b)
  pointer += 2
  return true
}

main()
