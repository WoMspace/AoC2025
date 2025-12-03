open System.Diagnostics

printfn "Advent of Code 2025, Day 02"
let args = System.Environment.GetCommandLineArgs()
let path = args[1]
let inputLine = System.IO.File.ReadAllText path

// part 1
let timer = Stopwatch.StartNew()
let mutable solution= 0UL
for rangeString in inputLine.Split ',' do
    let split = rangeString.Split('-')
    let min = uint64 split[0]
    let max = uint64 split[1] + 1UL
    for id in min..max+1UL do
        let id_string = string id
        let halfway = id_string.Length / 2
        let first_half = id_string[..halfway-1]
        let second_half = id_string[halfway..]
        // printfn $"{id_string} = {first_half}+{second_half}"
        if first_half = second_half then
            solution <- solution + id

let part1_time = timer.Elapsed
printfn $"Day 02 Part 1: {solution} in {part1_time.TotalMicroseconds}µs"

// part 2
timer.Restart()
solution <- 0UL
for rangeString in inputLine.Split ',' do
    let split = rangeString.Split('-')
    let min = uint64 split[0]
    let max = uint64 split[1] + 1UL
    let bad_ids = seq {
        for id in min..max+1UL do
            let id_string = string id
            for i in 1..id_string.Length-1 do
                if id_string.Length % i = 0 then
                    let substring = id_string[..(i-1)]
                    let reconstructed_string = String.replicate (id_string.Length/i) substring
                    if reconstructed_string = id_string then
                        // printfn $"invalid ID: {id_string} is {substring}"
                        yield id
                        // solution <- solution + id
    }
    let sum = bad_ids |> Seq.distinct |> Seq.sum
    solution <- solution + sum

let part2_time = timer.Elapsed
printfn $"Day 02 Part 2: {solution} in {part2_time.TotalMicroseconds}µs"