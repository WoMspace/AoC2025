use std::ops::RangeInclusive;

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
    let (ranges_str, available_ingredients_str) =input.split_once("\n\n").unwrap();
    let mut ranges = Vec::new();
    for range_str in ranges_str.lines() {
        let (start, end) = range_str.split_once('-').unwrap();
        let range: RangeInclusive<i64> = start.parse().unwrap()..=end.parse().unwrap();
        ranges.push(range);
    }
    for ingredient_str in available_ingredients_str.lines() {
        let ingredient: i64 = ingredient_str.parse().unwrap();
        'check_ranges: for range in &ranges {
            if range.contains(&ingredient) {
                solution += 1;
                break 'check_ranges
            }
        }
    }

    let part1_time = now.elapsed().as_micros();
    println!("Day 05 part 1: {solution} in {part1_time}µs");
}