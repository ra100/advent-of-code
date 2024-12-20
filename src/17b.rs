const PROGRAM: [u64; 16] = [2, 4, 1, 5, 7, 5, 4, 5, 0, 3, 1, 6, 5, 5, 3, 0];

fn main() {
    let mut program: Vec<u64> = PROGRAM.clone().to_vec();
    let max_index = 17;
    program.reverse();

    let mut possible_a = vec![vec![]; 16];
    possible_a[0] = vec![0];

    for index in 1..max_index {
        for expected_a in possible_a[index - 1].clone() {
            for i in 0..7 {
                let expected_d = program[index - 1];
                let a = expected_a * 8 + i;
                let d = (((a % 8) ^ 5) ^ (a / 2u64.pow(((a % 8) ^ 5) as u32)) ^ 6) % 8;
                let a1 = a / 8;

                if a1 == expected_a && d == expected_d {
                    println!(
                        "index: {} i: {} ea: {} ed: {} a1: {} a: {} d: {}",
                        index, i, expected_a, expected_d, a1, a, d
                    );
                    possible_a[index].push(a);
                }
            }
        }
    }

    println!("out: {:?}", possible_a);
}
