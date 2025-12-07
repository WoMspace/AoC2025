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
    let grid_backup = grid.clone(); // save the unmodified grid for part 2

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
    }

    let part1_time = now.elapsed().as_micros();
    println!("Day 07 part 1: {solution} in {part1_time}µs");


    // part 2
    let now = std::time::Instant::now();
    // do the same as part 1, but it's also a goobey floodfill
    let mut flood = vec![vec![0u64; width]; height];
    let splitters = grid_backup; // nicer name so it isn't confusing up top
    let x = splitters[0].iter().position(|c| *c == 'S').unwrap();
    flood[1][x] += 1;
    for y in (2..height).step_by(2) {
        for x in 0..width {
            if flood[y-1][x] > 0 {
                match splitters[y][x] {
                    '.' => flood[y+1][x] += flood[y-1][x],
                    '^' => {
                        flood[y+1][x-1] += flood[y-1][x];
                        flood[y+1][x+1] += flood[y-1][x];
                    }
                    _ => {}
                }
            }
        }
    }
    solution = flood[height-1].iter().sum();

    let part2_time = now.elapsed().as_micros();
    println!("Day 07 part 2: {solution} in {part2_time}µs");
}