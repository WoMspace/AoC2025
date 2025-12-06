#[derive(PartialEq)]
#[derive(Debug)]
pub enum Operation {
    Multiply,
    Addition
}
#[derive(Debug)]
pub struct Problem {
    pub numbers: Vec<isize>,
    pub operation: Option<Operation>
}

fn main() {println!("Advent of Code 2025, Day 05");
    let path = match std::env::args().nth(1) {
        Some(a) => a,
        None => {
            eprintln!("Missing required argument INPUT_PATH");
            std::process::exit(1);
        }
    };
    let input = match std::fs::read_to_string(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error reading file: {e}");
            std::process::exit(1)
        }
    };

    // part 1
    let now = std::time::Instant::now();
    let mut solution = 0;
    let lines: Vec<&str> = input.lines().collect();
    let number_of_problems = lines[0].split(' ').count();
    let rows = lines.len();
    let mut problems = Vec::<Problem>::with_capacity(number_of_problems);
    for _ in 0..number_of_problems {
        let problem = Problem { numbers: Vec::new(), operation: None };
        problems.push(problem);
    }
    for row in 0..=(rows-2) {
        let line = lines[row];
        let numbers: Vec<isize> = line.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect();
        for (i, num) in numbers.iter().enumerate() {
            problems[i].numbers.push(*num);
        }
    }
    // let _ = lines[rows-1].split_ascii_whitespace().enumerate().map(
    //     |(i, s)| problems[i].operation = match s {
    //         "+" => Some(Operation::Addition),
    //         "*" => Some(Operation::Multiply),
    //         e => panic!("unknown operation '{e}'")
    // });
    for (i, op) in lines.last().unwrap().split_ascii_whitespace().enumerate() {
        problems[i].operation = match op {
            "+" => Some(Operation::Addition),
            "*" => Some(Operation::Multiply),
            e => panic!("unknown operation '{e}'")
        }
    }

    for problem in &problems {
        // print!("{problem:?} = ");
        let mut result = 0;
        if problem.operation == Some(Operation::Addition) {
            for number in &problem.numbers {
                result += number;
            }
        } else if problem.operation == Some(Operation::Multiply) {
            result = 1;
            for number in &problem.numbers {
                result *= number;
            }
        }
        solution += result;
        // println!("{result}");
    }

    let part1_time = now.elapsed().as_micros();
    println!("Day 05 part 1: {solution} in {part1_time}µs");
}
