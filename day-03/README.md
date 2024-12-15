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
- I think I will need the index of every `mul(number,number)` match
- `get(0)` will return a match like this: `Match { start: 1, end: 9, string: "mul(2,4)" }`
- `get(0).unwrap().start()` will return the start index of the match: `1`
- `get(0).unwrap().as_string()` will return the substring we are looking for: `mul(2,4)`
```rust
use regex::Regex;
fn process_input(input: &str) {
  let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
  for capture in re.captures_iter(&input) { println!("{:?}", capture.get(0).unwrap()); }
}
```
- The idea is to compare the indexes of `do()`, `don´t()` and `mul(number,number)`
- The following will print: `do(): [59], don't(): [20]`
```rust
use regex::Regex;
fn process_input(input: &str) {
  let re_do = Regex::new(r"do\(\)").unwrap();
  let re_dont = Regex::new(r"don\'t\(\)").unwrap();
  let mut dos: Vec<usize> = Vec::new();
  let mut donts: Vec<usize> = Vec::new();
  for capture in re_do.captures_iter(&input) { dos.push(capture.get(0).unwrap().start()); }
  for capture in re_dont.captures_iter(&input) { donts.push(capture.get(0).unwrap().start()); }
  println!("do(): {:?}, don't(): {:?}", dos, donts);
}
```
- We will add `filter` after the iterator to filter out the disabled instructions
- Just like before we save all the valid instructiones indexes with `start`
- We will use `index` to get the closest `do()` and `don't()`
- With `filter` we will only keep the `do()` and `don't()` of index smaller than an instruction's index
- And then we will only keep the biggest value with `max`
- `max` returns an `Option` which can be `Some(T)` when there is a valid value, otherwise `None`
- The following will print:
```
1
None
None
28
None
Some(20)
48
None
Some(20)
64
Some(59)
Some(20)
```
- `1 None None` means that we have a `mul(number,number)` at index 1 and there are none `do()` nor `don't()` before it
- In this case we allow the multiplication
- `28 None Some(20)` means that we have a `don't()` at index 20, which disables the multiplication
```rust
fn process_input(input: &str) {
  // ...
  let sum: usize = re
    .captures_iter(&input)
    .filter(|capture| {
      let index = capture.get(0).unwrap().start();
      println!("{:?}", index);
      let last_do = dos.iter().filter(|&&i| i < index).max();
      println!("{:?}", last_do);
      let last_dont = donts.iter().filter(|&&i| i < index).max();
      println!("{:?}", last_dont);
      true
    })
    .map(|group| {
      group[1].parse::<usize>().unwrap() * group[2].parse::<usize>().unwrap()
    })
    .sum();
  println!("{}", sum);
}
```
- Looking at the previous result we notice that we have four posible outcomes
| do() | don't | return |
| --- | --- | --- |
| Some(index 1) | Some(index 2) | true if index 1 > index 2 |
| Some(index) | None | true |
| None | Some(index) | false |
| None | None | true |
```rust
.filter(|capture| {
  let index = capture.get(0).unwrap().start();
  let last_do = dos.iter().filter(|&&i| i < index).max();
  let last_dont = donts.iter().filter(|&&i| i < index).max();

  if last_do.is_some() && last_dont.is_some() { last_do > last_dont }
  else if last_do.is_some() && last_dont.is_none() { return true; }
  else if last_do.is_none() && last_dont.is_some() { return false; }
  else { return true; }
})
```
