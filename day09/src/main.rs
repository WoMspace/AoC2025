fn main() {
    println!("Advent of Code 2025, Day 09");
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
    let coords: Vec<(usize, usize)> = input.lines()
        .map(|s| s.split_once(',').unwrap())
        .map(|(s1, s2)| (s1.parse().unwrap(), s2.parse().unwrap()))
        .collect();
    let mut solution = 0;
    for coord1 in &coords {
        for coord2 in &coords {
            let area = coord1.0.abs_diff(coord2.0+1) * coord1.1.abs_diff(coord2.1+1);
            if area > solution {
                solution = area;
            }
        }
    }

    let part1_time = now.elapsed().as_micros();
    println!("Day 09 part 1: {solution} in {part1_time}µs");
}
