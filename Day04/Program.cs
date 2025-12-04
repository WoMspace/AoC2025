using System.Text;

Console.WriteLine("Advent of Code 2025, Day 04");
string path = args[0];
var inputLines = File.ReadAllLines(path);

// part 1
var timer = System.Diagnostics.Stopwatch.StartNew();
int solution = 0;
var width = inputLines[0].Length;
var height = inputLines.Length;
int[,] nearby = new int[width, height];
scatter(inputLines, nearby);

for(int y = 0; y < height; y += 1) {
	for(int x = 0; x < width; x += 1) {
		if(inputLines[y][x] == '@' && nearby[x, y] < 4) solution += 1;
	}
}

var part1_time = timer.Elapsed;
Console.WriteLine($"Day 04 Part 1: {solution} in {part1_time.TotalMicroseconds}µs");

// part 2
solution = 0;
timer.Restart();
string[] state = inputLines;
int removed;
do {
	removed = 0;
	nearby = new int[width, height];
	scatter(state, nearby);
	for(int y = 0; y < height; y += 1) {
		StringBuilder row = new();
		for(int x = 0; x < width; x += 1) {
			if(state[y][x] == '@') {
				if(nearby[x, y] < 4) {
					// remove this one
					removed += 1;
					solution += 1;
					row.Append('.');
				} else row.Append('@');
			} else row.Append('.');
		}

		state[y] = row.ToString();
	}
	// Console.WriteLine($"Removed {removed} bales");
} while(removed > 0);

var part2_time = timer.Elapsed;
Console.WriteLine($"Day 04 Part 2: {solution} in {part2_time.TotalMicroseconds}µs");

void scatter(string[] source, int[,] destination) {
	int max_x = source[0].Length;
	int max_y = source.Length;
	for(int y = 0; y < max_y; y += 1) {
		for(int x = 0; x < max_x; x += 1) {
			if(inputLines[y][x] == '@') {
				// increment surrounding pixels, ordered from top left clockwise
				if(x > 0 && y > 0) destination[x - 1, y - 1] += 1;              // ↖
				if(y > 0) destination[x, y - 1] += 1;                           // ↑
				if(x < max_x-1 && y > 0) destination[x + 1, y - 1] += 1;        // ↗
				if(x < max_x-1) destination[x + 1, y] += 1;                     // →
				if(x < max_x-1 && y < max_y-1) destination[x + 1, y + 1] += 1;  // ↘
				if(y < max_y-1) destination[x, y + 1] += 1;                     // ↓
				if(x > 0 && y < max_y-1) destination[x - 1, y + 1] += 1;        // ↙
				if(x > 0) destination[x - 1, y] += 1;                           // ←
			}
		}
	}
}