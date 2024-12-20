// use std::time::Instant;

const PROGRAM: [u64; 16] = [2, 4, 1, 5, 7, 5, 4, 5, 0, 3, 1, 6, 5, 5, 3, 0];
// const PROGRAM: [u64; 9] = [3, 4, 3, 1, 7, 6, 5, 6, 0]; // 63_281_501

fn main() {
    // let mut start = Instant::now();

    let mut out: Vec<u64> = vec![];
    let mut a: u64;
    // let mut b: u64;
    // let mut c: u64;

    let initial_a = 1703436411430;

    out.clear();
    a = initial_a;

    loop {
        // 5 5
        let d = (((a % 8) ^ 5) ^ (a / 2u64.pow(((a % 8) ^ 5) as u32)) ^ 6) % 8;

        a = a / 8;

        out.push(d);

        // 3 0
        if a == 0 {
            break;
        };
    }

    println!("out: {:?}", out);
}

// fn main() {
//     let mut start = Instant::now();

//     let mut out: Vec<u64> = vec![];
//     let mut a: u64;
//     // let mut b: u64;
//     // let mut c: u64;

//     let mut iteration;
//     let mut found;

//     for i in 63_281_501..63_281_502 {
//         let initial_a = i;

//         if i % 1_000_000 == 0 {
//             let duration = start.elapsed();
//             println!("{}: {:?}", i / 1_000_000, duration);
//             start = Instant::now();
//         }

//         out.clear();
//         a = initial_a;

//         iteration = 0;
//         found = false;

//         loop {
//             // 2 4
//             // b = (a % 8) ^ 5;

//             // 1 5
//             // b = b ^ 5;

//             // 7 5
//             // c = a / base.pow(((a % 8) ^ 5) as u32);

//             // 4 5
//             // b = ((a % 8) ^ 5) ^ (a / base.pow(((a % 8) ^ 5) as u32));

//             // 0 3

//             // 1 6
//             // b = ((a % 8) ^ 5) ^ (a / base.pow(((a % 8) ^ 5) as u32)) ^ 6;

//             // 5 5
//             let d = (((a % 8) ^ 5) ^ (a / 2u64.pow(((a % 8) ^ 5) as u32)) ^ 6) % 8;

//             a = a / 8;

//             // if d != PROGRAM[iteration] {
//             //     break;
//             // }

//             out.push(d);

//             // 3 0
//             if a == 0 {
//                 if iteration == PROGRAM.len() - 1 {
//                     found = true;
//                 }
//                 break;
//             };

//             iteration += 1;
//         }

//         if found {
//             println!("Found a match: {}", initial_a);
//             break;
//         }
//     }

//     println!("out: {:?}", out);
// }
