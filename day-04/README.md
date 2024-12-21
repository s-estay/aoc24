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
- And scan it looking if the `XMAS` forms in any direction

|  S  |     |     |  S  |     |     |  S  |
| --- | --- | --- | --- | --- | --- | --- |
|     |  A  |     |  A  |     |  A  |     |
|     |     |  M  |  M  |  M  |     |     |
|  S  |  A  |  M  |  X  |  M  |  A  |  S  |
|     |     |  M  |  M  |  M  |     |     |
|     |  A  |     |  A  |     |  A  |     |
|  S  |     |     |  S  |     |     |  S  |

- After this point it took me more time to find the solution than I would like to admit

### Create matrix
- Save test input to vector of strings
- The followng will print
```
10 x 10
["MMMSXXMASM", "MSAMXMSMSA", "AMXSXMAAMM", "MSAMASMSMX", "XMASAMXAMM", "XXAMMXXAMA", "SMSMSASXSS", "SAXAMASAAA", "MAMMMXMMMM", "MXMXAXMASX"]
```
```rust
fn process_input(input: &str) {
  let mut grid: Vec<String> = Vec::new();
  for line in input.lines() { grid.push(line.to_string()); }
  let rows = grid.len();
  let cols = grid[0].len();
  println!("{:?} x {:?}", rows, cols);
  println!("{:?}", grid);
}
fn main() {
  let file = include_str!("test-input.txt");
  process_input(file);
}
```

### Access every element
- Get index and the letter in that position
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

### Directions
- We have two loops that go through rows and columns
- Inside that loop we will have another loop that will allow us to travel in any given direction
- If an `X` has coordinates (i, j), then all the possible combinations will look like this
- Welcome back to linear algebra

| S (i-3, j-3) |              |              | S (i-3, j) |              |              | S (i-3, j+3) |
| ------------ | ------------ | ------------ | ---------- | ------------ | ------------ | ------------ |
|              | A (i-2, j-2) |              | A (i-2, j) |              | A (i-2, j+2) |              |
|              |              | M (i-1, j-1) | M (i-1, j) | M (i-1, j+1) |              |              |
| S (i, j-3)   | A (i, j-2)   | M (i, j-1)   | X (i, j)   | M (i, j+1)   | A (i, j+2)   | S (i, j+3)   |
|              |              | M (i+1, j-1) | M (i+1, j) | M (i+1, j+1) |              |              |
|              | A (i+2, j-2) |              | A (i+2, j) |              | A (i+2, j+2) |              |
| S (i+3, j-3) |              |              | S (i+3, j) |              |              | S (i+3, j+3) |

- We can translate the directions to the cardinal points

| NW | N | NE |
| --- | --- | --- |
| W | X | E |
| SW | S | SE |

- Travel N: X at (0,0), M at (-1,0), A at (-2,0), S at (-3,0)
- Travel S: X at (0,0), M at (+1,0), A at (+2,0), S at (+3,0)
- Travel E: X at (0,0), M at (0,+1), A at (0,+2), S at (0,+3)
- Travel W: X at (0,0), M at (0,-1), A at (0,-2), S at (0,-3)
- Travel NE: X at (0,0), M at (-1,+1), A at (-2,+2), S at (-3,+3)
- Travel NW: X at (0,0), M at (-1,-1), A at (-2,-2), S at (-3,-3)
- Travel SE: X at (0,0), M at (+1,+1), A at (+2,+2), S at (+3,+3)
- Travel SW: X at (0,0), M at (+1,-1), A at (+2,-2), S at (+3,-3)

### Loop XMAS
- And inside that loop we will have another loop that will give us a tuple with the index and letter of XMAS
- The following will print
```
(0, 'X')
(1, 'M')
(2, 'A')
(3, 'S')
```
```rust
fn main() {
  let xmas = "XMAS";
  for i in xmas.chars().enumerate() {
    println!("{:?}", i);
  }
}
```
- But I think is more useful to get the index and the letter directly in a variable
- The following will print
```
0 'X'
1 'M'
2 'A'
3 'S'
```
```rust
fn main() {
  let xmas = "XMAS";
  for (i, ch) in xmas.chars().enumerate() {
    println!("{:?} {:?}", i, ch);
  }
}
```

### How to move
- To move to any given direction, we will use unit vectors
- An unit vector will give the direction but not how much we need to move
- For that we will use the index of XMAS that increments for every letter
- To move one letter to the right (E), multiply the current position with the unit vector (0,1)
- To move two letters to the right, (0,1) * the index of the letter A in XMAS (2)

### How to know when we get a match
- We get a match when all letter match
- A for-loop needs some logic to verify a match, count how many matches we have thus fur, and finally give a result
- An easier way is to use the iterator transform `all()`, which *tests if every element of the iterator matches a predicate*
- *all() takes a closure that returns true or false*

### Putting all together
- Create matrix
- For the directions, create an array to tuples of type `isize` since some directions are negative numbers
- Create mutable variable `count`
- Scan matrix by rows and columns
- For every direction
- For every index and letter in XMAS
- Use `all()` to check if all the conditions match every letter of XMAS
- Calculate the next position by adding the vector direction * current index of letter XMAS, to the current position (r, c)
- Use the next position to check if the letter found in that position matches the current letter in XMAS
- Since the directions are of type `isize`, we need to type cast the index variables
- When `all()` return true, the conditions in the if statement will be true, which will allow us to increment the counter
- All this is very different from the C language I'm used to program in
```rust
fn process_input(input: &str) {
  let mut grid: Vec<String> = Vec::new();
  for line in input.lines() { grid.push(line.to_string()); }
  let rows = grid.len();
  let cols = grid[0].len();
  let xmas = "XMAS";
  //                                       N       NE      E     SE      S      SW      W        NW
  let directions: [(isize, isize); 8] = [(-1,0), (-1,1), (0,1), (1,1), (1,0), (1,-1), (0,-1), (-1,-1)];
  let mut count = 0;
  for r in 0..rows {
    for c in 0..cols {
      for &(dx, dy) in &directions {
        if xmas.chars().enumerate().all(|(i, ch)| {
          let nx = r as isize + i as isize * dx;
          let ny = c as isize + i as isize * dy;
          nx >= 0 &&
          ny >= 0 &&
          (nx as usize) < cols &&
          (ny as usize) < rows &&
          grid[nx as usize].as_bytes()[ny as usize] as char == ch
        }) {
          count += 1;
        }
      }
    }
  }
  println!("{count}");
}
```
