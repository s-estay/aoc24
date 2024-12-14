## Setup
- `cargo new day-03 --vcs none`
- `cd day-03`
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
- Test input: `xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))`
- Test input expected result: (2 * 4) + (5 * 5) + (11 * 8) + (8 * 5) = 161
- We are looking for the pattern `mul(number,number)`
- So this is a great oportunity to use a regular expression
- Go to [regex101](https://regex101.com) and find a regex for `mul(number,number)`
- This regex `mul\(\d+,\d+\)` will match the pattern
- Using parenthesis around the digits like this `(\d+)` will create a group
- A group allow us to refer to an isolated part of a regex by id
- The whole match will have id `id[0]`, the first number `id[1]` and the second `id[2]`
- This regex `mul\((\d+),(\d+)\)` will find the pattern and create a group for each number
- Rust implements regex with the crate [regex](https://crates.io/crates/regex)
- Add `regex = "1.11.1"` under `[dependencies]` in `Cargo.toml`
- Add `use regex::Regex;` at the top of `part1.rs` to be able to use the crate
- Define a new regex with `Regex::new`
- `new` will return a `Result<Regex, Error>` but I wasn't able to handle the error with a `match`
- [Using unwrap() in Rust is okay](https://blog.burntsushi.net/unwrap/)
```rust
use regex::Regex;
fn process_input(input: &str) {
  println!("{}", input);
  let _re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
}
fn main() {
  let file = include_str!("test-input.txt");
  process_input(file);
}
```
- Now lets see if our regex is able to find the groups in the test input
- `captures_iter` will create an iterator over all the matches found in `input`
- With `map` be can access the groups
- Each group is parsed to `usize` and then multiplied to the other one
- The elements in the iterator are finally added together with `sum`
```rust
use regex::Regex;
fn process_input(input: &str) {
  let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
  let sum: usize = re.captures_iter(&input).map(|group| {
    group[1].parse::<usize>().unwrap() * group[2].parse::<usize>().unwrap()
  }).sum();
  println!("{}", sum);
}
fn main() {
  // ...
}
```

## Part 2
- New test input: `xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))`
- The `do()` instruction enables future mul instructions
- The `don't()` instruction disables future mul instructions
- `do()` regex: `do\(\)`
- `don't()` regex: `don\'t\(\)`
