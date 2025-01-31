use std::time::Instant;

const PROGRAM: [u64; 16] = [2, 4, 1, 5, 7, 5, 4, 5, 0, 3, 1, 6, 5, 5, 3, 0]; // 63_281_501
const PROGRAM_LENGTH: usize = 16;
const REPEAT: u64 = 196608;

fn main() {
    let mut start = Instant::now();

    // let program = vec![0, 3, 5, 4, 3, 0];
    let mut register_a = 0;
    let mut register_b = 0;
    let mut register_c = 0;

    let mut pointer: usize;

    let mut registers = [&mut register_a, &mut register_b, &mut register_c];
    let mut output: Vec<u64> = Vec::new();
    let mut possible: bool;

    let base: u64 = 2;
    let mut initial_value = 0;

    // 63_281_501
    for i in 1..2 {
        possible = false;

        // if i % 10 == 0 {
        //     let duration = start.elapsed();
        //     println!("{}: {:?}", i / 10, duration);
        //     start = Instant::now();
        // }

        for j in 0..16 {
            initial_value = initial_value + base.pow(j) + 1;

            output.clear();
            possible = false;

            pointer = 0;
            *registers[0] = initial_value;
            *registers[1] = 0;
            *registers[2] = 0;

            while pointer < PROGRAM_LENGTH {
                let operand = PROGRAM[pointer + 1];
                match PROGRAM[pointer] {
                    0 => adv(&mut registers, operand, &mut pointer),
                    1 => bxl(&mut registers, operand, &mut pointer),
                    2 => bst(&mut registers, operand, &mut pointer),
                    3 => jnz(&mut registers, operand, &mut pointer),
                    4 => bxc(&mut registers, operand, &mut pointer),
                    5 => {
                        out(&mut registers, operand, &mut pointer, &mut output);
                        let len = output.len();
                        // println!("{}: {:?}", initial_value, output);
                        if len == 0 {
                            continue;
                        }
                        if len >= PROGRAM_LENGTH {
                            break;
                        }
                        let idx = len - 1;

                        if PROGRAM[idx] != output[idx] {
                            break;
                        }

                        if pointer >= PROGRAM_LENGTH && len == PROGRAM_LENGTH {
                            possible = true;
                            break;
                        }
                    }
                    6 => bdv(&mut registers, operand, &mut pointer),
                    7 => cdv(&mut registers, operand, &mut pointer),
                    _ => {
                        break;
                    }
                };
            }

            println!("{}: {:?}", initial_value, output);

            if !possible {
                continue;
            }

            // let mut found_match = true;
            // for i in 0..output.len() {
            //     if program[i] != output[i] {
            //         found_match = false;
            //         break;
            //     }
            // }
            // if found_match {
            println!("Found a match: {}, {:?}", initial_value, output);
            break;
            // }
        }
        if !possible {
            continue;
        }
        break;
    }
    println!("{:?}", output);
}

fn get_combo_operand(registers: &mut [&mut u64; 3], combo_operand: u64) -> u64 {
    if combo_operand >= 7 {
        panic!("Invalid combo operand: {}", combo_operand);
    }
    if combo_operand <= 3 {
        combo_operand
    } else {
        *registers[(combo_operand - 4) as usize]
    }
}

/**
 * The adv instruction (opcode 0) performs division. The numerator is the value
 * in the A register. The denominator is found by raising 2 to the power of the
 * instruction's combo operand. (So, an operand of 2 would divide A by 4 (2^2);
 * an operand of 5 would divide A by 2^B.) The result of the division operation
 * is truncated to an integer and then written to the A register.
 */
fn adv(registers: &mut [&mut u64; 3], combo_operand: u64, pointer: &mut usize) {
    let a = *registers[0];
    let b = get_combo_operand(registers, combo_operand);

    let base: u64 = 2;

    *registers[0] = a / base.pow(b as u32);
    *pointer += 2;
}

/**
 * The bxl instruction (opcode 1) calculates the bitwise XOR of register B and
 * the instruction's literal operand, then stores the result in register B.
 */
fn bxl(registers: &mut [&mut u64; 3], literal_operand: u64, pointer: &mut usize) {
    *registers[1] = *registers[1] ^ literal_operand;
    *pointer += 2;
}

/**
 * The bst instruction (opcode 2) calculates the value of its combo operand modulo 8
 * (thereby keeping only its lowest 3 bits), then writes that value to the B register.
 */
fn bst(registers: &mut [&mut u64; 3], combo_operand: u64, pointer: &mut usize) {
    *registers[1] = get_combo_operand(registers, combo_operand) % 8;
    *pointer += 2;
}

/**
 * The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the A register is not zero, it jumps
 * by setting the instruction pointer to the value of its literal operand; if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
 */
fn jnz(registers: &mut [&mut u64; 3], literal_operand: u64, pointer: &mut usize) {
    let a = *registers[0];

    if a == 0 {
        *pointer += 2;
        return;
    }

    *pointer = literal_operand as usize;
}

/**
 * The bxc instruction (opcode 4) calculates the bitwise XOR of register B and
 * register C, then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
 */
fn bxc(registers: &mut [&mut u64; 3], _: u64, pointer: &mut usize) {
    *registers[1] = *registers[1] ^ *registers[2];
    *pointer += 2;
}

/**
 * The out instruction (opcode 5) calculates the value of its combo operand modulo 8,
 * then outputs that value. (If a program outputs multiple values, they are separated by commas.)
 */
fn out(
    registers: &mut [&mut u64; 3],
    combo_operand: u64,
    pointer: &mut usize,
    output: &mut Vec<u64>,
) {
    let a = get_combo_operand(registers, combo_operand) % 8;

    output.push(a);
    *pointer += 2;
}

/**
 * The bdv instruction (opcode 6) works exactly like the adv instruction except
 * that the result is stored in the B register. (The numerator is still read from the A register.)
 */
fn bdv(registers: &mut [&mut u64; 3], combo_operand: u64, pointer: &mut usize) {
    let a = *registers[0];
    let b = get_combo_operand(registers, combo_operand);

    let base: u64 = 2;

    *registers[1] = a / base.pow(b as u32);
    *pointer += 2;
}

/**
 * The cdv instruction (opcode 7) works exactly like the adv instruction except
 * that the result is stored in the C register. (The numerator is still read from the A register.)
 */
fn cdv(registers: &mut [&mut u64; 3], combo_operand: u64, pointer: &mut usize) {
    let a = *registers[0];
    let b = get_combo_operand(registers, combo_operand);

    let base: u64 = 2;

    *registers[2] = a / base.pow(b as u32);
    *pointer += 2;
}
