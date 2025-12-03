Console.WriteLine("Advent of Code 2025, Day 03");
string path = args[0];
var inputLines = System.IO.File.ReadAllLines(path);

// part 1
var timer = System.Diagnostics.Stopwatch.StartNew();
int solution = 0;
foreach(string bank in inputLines) {
	// convert string to list of ints
	List<int> joltages = new();
	foreach(char c in bank) {
		joltages.Add(c - '0');
	}
	// find highest non-last number
	int highest = 0;
	int highest_position = 0;
	for(int i = 0; i < joltages.Count - 1; i++) {
		if(joltages[i] > highest) {
			highest = joltages[i];
			highest_position = i+1;
		}
	}
	int tens = highest;
	// find highest number after the current number
	int ones = joltages[highest_position..].Max();
	string enabled_batteries = $"{tens}{ones}";
	// Console.WriteLine($"Chose {enabled_batteries} from {bank}");
	solution += int.Parse(enabled_batteries);
}

var part1_time = timer.Elapsed;
Console.WriteLine($"Day 03 Part 1: {solution} in {part1_time.TotalMicroseconds}µs");

timer.Restart();
ulong solution2 = 0;
foreach(string bank in inputLines) {
	string enabled_batteries = String.Empty;
	int last_index = 0;
	// `i` is the length from the end of the string
	for(int i = 11; i >= 0; i--) {
		int max = 0;
		for(int j = last_index; j < bank.Length - i; j++) {
			if(bank[j]-'0' > max) {
				max = bank[j] - '0';
				last_index = j+1;
				if(max == 9) { break; }
			}
		}

		enabled_batteries += max;
	}
	// Console.WriteLine($"Enabled {enabled_batteries} from {bank}");
	solution2 += ulong.Parse(enabled_batteries);
}
var part2_time = timer.Elapsed;
Console.WriteLine($"Day 03 Part 2: {solution2} in {part2_time.TotalMicroseconds}µs");
