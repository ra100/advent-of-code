use std::fs;

fn load_file(file_path: &str) -> Result<String, std::io::Error> {
    return Ok(fs::read_to_string(file_path).expect("Something went wrong reading the file"));
}

fn main() {
    let input_file = "src/01.txt";

    let contents = load_file(input_file);

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
