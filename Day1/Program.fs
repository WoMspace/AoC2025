open System.Diagnostics

let modulus (x, y) = (x % y + y) % y

printfn "Advent of Code 2025, Day 01"
let args = System.Environment.GetCommandLineArgs()
let path = args[1]
let inputLines = System.IO.File.ReadAllLines path

// part 1
let timer = Stopwatch.StartNew()
let mutable dial = 50
let mutable zeroCount = 0
for line in inputLines do
    let sign = match line[0] with
                | 'L' -> -1
                | 'R' -> 1
                | _ -> 0
    let distance = int line[1..]
    
    dial <- dial + (distance * sign)
    dial <- modulus(dial, 100)
    if dial = 0 then zeroCount <- zeroCount + 1
let part1_time = timer.Elapsed
printfn $"Part 1: {zeroCount} in {part1_time.TotalMicroseconds}µs"

// part 2
timer.Restart()
dial <- 50
zeroCount <- 0
for line in inputLines do
    let sign = match line[0] with
                | 'L' -> -1
                | 'R' -> 1
                | _ -> failwith "Invalid direction"
    let distance = int line[1..]
    for _ in 1..distance do
        dial <- dial + sign
        // match dial with
        // | -1 -> dial <- 99
        // | 100 -> dial <- 0
        // | _ -> ()
        dial <- modulus(dial, 100)
        if dial = 0 then zeroCount <- zeroCount + 1

let part2_time = timer.Elapsed
printfn $"Part 2: {zeroCount} in {part2_time.TotalMicroseconds}µs"