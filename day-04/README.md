## Setup
- `cargo new day-04 --vcs none`
- `cd day-04`
- `mkdir src/bin`
- `mv src/main.rs src/bin/part1.rs`
```rust
fn process_input(input: &str) {
  println!("{}", input);
}
fn main() {
  let file = include_str!("test-input.txt");
  process_input(file);
}
```
- copy test input and then `pbpaste > src/bin/test-input.txt`
- `cargo run --bin part1`

## Part 1
- My first thought was to start by scaning the test input using two regexs: `XMAS` and `SAMX`
- The following prints 5
```rust
use regex::Regex;
fn process_input(input: &str) {
  let xmas = Regex::new(r"XMAS").unwrap();
  let samx = Regex::new(r"SAMX").unwrap();
  let sum: usize = xmas.find_iter(&input).count() + samx.find_iter(&input).count();
  println!("{}", sum);
}
fn main() {
  let file = include_str!("test-input.txt");
  process_input(file);
}
```
- And then I realized that the vertical and diagonal scanning would requiere indexes
- So the correct approach is to convert the input to a matrix of letters
- Find every `X` and look if the `XMAS` forms around it going in all directions

|  S  |     |     |  S  |     |     |  S  |
| --- | --- | --- | --- | --- | --- | --- |
|     |  A  |     |  A  |     |  A  |     |
|     |     |  M  |  M  |  M  |     |     |
|  S  |  A  |  M  |  X  |  M  |  A  |  S  |
|     |     |  M  |  M  |  M  |     |     |
|     |  A  |     |  A  |     |  A  |     |
|  S  |     |     |  S  |     |     |  S  |

- If an `X` has coordinates (i, j), then the table will look like this
- Welcome back to linear algebra

| S (i-3, j-3) |              |              | S (i-3, j) |              |              | S (i-3, j+3) |
| ------------ | ------------ | ------------ | ---------- | ------------ | ------------ | ------------ |
|              | A (i-2, j-2) |              | A (i-2, j) |              | A (i-2, j+2) |              |
|              |              | M (i-1, j-1) | M (i-1, j) | M (i-1, j+1) |              |              |
| S (i, j-3)   | A (i, j-2)   | M (i, j-1)   | X (i, j)   | M (i, j+1)   | A (i, j+2)   | S (i, j+3)   |
|              |              | M (i+1, j-1) | M (i+1, j) | M (i+1, j+1) |              |              |
|              | A (i+2, j-2) |              | A (i+2, j) |              | A (i+2, j+2) |              |
| S (i+3, j-3) |              |              | S (i+3, j) |              |              | S (i+3, j+3) |

## Part 1 (start over)
- Save test input to vector of strings
- The followng will print
```
["MMMSXXMASM", "MSAMXMSMSA", "AMXSXMAAMM", "MSAMASMSMX", "XMASAMXAMM", "XXAMMXXAMA", "SMSMSASXSS", "SAXAMASAAA", "MAMMMXMMMM", "MXMXAXMASX"]
10
10
```
```rust
fn process_input(input: &str) {
  let mut grid: Vec<String> = Vec::new();
  for line in input.lines() { grid.push(line.to_string()); }
  let rows = grid.len();
  let cols = grid[0].len();
  println!("{:?}", grid);
  println!("{:?}", rows);
  println!("{:?}", cols);
}
fn main() {
  let file = include_str!("test-input.txt");
  process_input(file);
}
```
- Access every element and get its index
- The following will print
```
0, 0 : 'M'
0, 1 : 'M'
0, 2 : 'M'
0, 3 : 'S'
0, 4 : 'X'
0, 5 : 'X'
0, 6 : 'M'
0, 7 : 'A'
0, 8 : 'S'
0, 9 : 'M'
1, 0 : 'M'
1, 1 : 'S'
...
```
```rust
fn process_input(input: &str) {
  let mut grid: Vec<String> = Vec::new();
  for line in input.lines() { grid.push(line.to_string()); }
  let rows = grid.len();
  let cols = grid[0].len();
  for r in 0..rows {
    for c in 0..cols {
      println!("{r}, {c} : {:?}", grid[r].as_bytes()[c] as char);
    }
  }
}
```
