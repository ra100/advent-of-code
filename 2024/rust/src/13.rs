use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    p: (i64, i64),
}

const A_COST: i64 = 3;
const B_COST: i64 = 1;

fn main() {
    let content = fs::read_to_string("src/13.txt").expect("Failed to read file");

    let machines = parse_content(&content);

    let mut total_cost: i64 = 0;
    for machine in machines {
        // println!("{:?} {:?} {:?}", machine.a, machine.b, machine.p);
        let cost = find_lowest_cost(&machine);
        total_cost += cost;
        // println!("Cost: {}\n", cost);
    }

    println!("Total cost: {}", total_cost);
}

fn find_lowest_cost(machine: &Machine) -> i64 {
    let solutions = solve_diophantine_equation(machine);
    let x = solutions.0;
    let y = solutions.1;

    let cost = A_COST * x + B_COST * y;

    return cost;
}

fn solve_diophantine_equation(machine: &Machine) -> (i64, i64) {
    let a = machine.a.0;
    let b = machine.b.0;
    let c = machine.a.1;
    let d = machine.b.1;
    let v = machine.p.0;
    let w = machine.p.1;

    let determinant = a * d - b * c;
    if determinant == 0 {
        return (0, 0); // No unique solution exists
    }

    // Use Cramer's rule to solve for y
    let determinant_y = v * c - w * a;
    if determinant_y % determinant != 0 {
        return (0, 0); // No integer solution
    }
    let y = (determinant_y / determinant) * -1;

    // Use Cramer's rule to solve for x
    let determinant_x = v * d - w * b;
    if determinant_x % determinant != 0 {
        return (0, 0); // No integer solution
    }
    let x = determinant_x / determinant;

    // Ensure non-negative solutions
    if x >= 0 && y >= 0 {
        (x, y)
    } else {
        (0, 0)
    }
}

fn parse_content(content: &str) -> Vec<Machine> {
    let mut machines = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    for chunk in lines.chunks(4) {
        if chunk.len() < 3 {
            continue;
        }

        let a = parse_coordinates(chunk[0], r"X\+(\d+), Y\+(\d+)");
        let b = parse_coordinates(chunk[1], r"X\+(\d+), Y\+(\d+)");
        let p = parse_coordinates(chunk[2], r"X=(\d+), Y=(\d+)");

        machines.push(Machine {
            a,
            b,
            p: (p.0 + 10000000000000, p.1 + 10000000000000),
        });
    }

    machines
}

fn parse_coordinates(line: &str, pattern: &str) -> (i64, i64) {
    let re = Regex::new(pattern).unwrap();
    let caps = re.captures(line).unwrap();

    let x = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
    let y = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();

    (x, y)
}
