use std::fs;

fn main() {
    let contents = fs::read_to_string("src/01.txt").expect("Something went wrong reading the file");

    let (mut data1, mut data2): (Vec<i32>, Vec<i32>) = contents
        .lines()
        .map(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|num_str| num_str.parse::<i32>().unwrap())
                .collect();

            (numbers[0], numbers[1])
        })
        .unzip();

    data1.sort();
    data2.sort();

    let result: i32 = data1
        .iter()
        .zip(&data2)
        .map(|(first, second)| (first - second).abs())
        .sum();

    println!("result: {:?}", result);
}
