#[allow(unused_imports)]
use std::time::Duration;

fn main() {
    println!("Advent of Code 2025, Day 05");
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
    let width = input.lines().nth(0).unwrap().len();
    let height = input.lines().count();
    let mut grid = vec![vec!['.'; width]; height];
    let input_chars: Vec<char> = input.chars().filter(|c| *c != '\n').collect();
    for y in 0..height {
        for x in 0..width {
            grid[y][x] = input_chars[x + y*width]
        }
    }

    // grid[][] now has everything :)
    'going_down: for y in (0..height).step_by(2) {
        for x in 0..width {
            if grid[y][x] == 'S' {
                // start point
                grid[y+1][x] = '|';
                continue 'going_down
            } else if y > 0 && grid[y-1][x] == '|' {
                if grid[y][x] == '^' {
                    grid[y+1][x-1] = '|';
                    grid[y+1][x+1] = '|';
                    solution += 1;
                } else {
                    grid[y+1][x] = '|';
                }
            }
        }
        // print_grid(&grid);
        // std::thread::sleep(Duration::from_millis(100))
    }

    let part1_time = now.elapsed().as_micros();
    println!("Day 07 part 1: {solution} in {part1_time}µs");
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<char>>) {
    let mut final_string = String::new();
    for row in grid {
        for c in row {
            final_string.push(*c);
        }
        final_string.push('\n');
    }
    let footer = String::from_utf8(vec![b'_'; grid[0].len()]).unwrap();
    final_string.push_str(&footer);
    println!("{final_string}");
}