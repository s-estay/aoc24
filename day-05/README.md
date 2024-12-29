## Setup
- `cargo new day-05 --vcs none`
- `cd day-05`
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
- `touch README.md`

## Part 1
- The test input consists of two parts that need to be processed and stored
- First store the input in a vector, where every lines is an element of a vector
- `lines()` will create an iterator over the lines of input
- `collect()` will create a collection out of an iterator
- The following will print
```
["47|53", "97|13", (...), "", "75,47,61,53,29", "97,61,53,29,13", (...)]
```
```rust
fn process_input(input: &str) {
  let lines: Vec<_> = input.lines().collect();
  println!("{:?}", lines);
}
fn main() {
  let file = include_str!("test-input.txt");
  process_input(file);
}
```
- We then split the vector where the empty line is
- This is the same place where the vector's empty element is located
- `split()` will create an iterator over subslices separated by the elements that match a given condition, in this case `is_empty()`
- The following will print
```
[["47|53", "97|13", (...)], ["75,47,61,53,29", "97,61,53,29,13", (...)]]
```
```rust
fn process_input(input: &str) {
  let lines: Vec<_> = input.lines().collect();
  let lines: Vec<_> = lines.split(|line| line.is_empty()).collect();
  println!("{:?}", lines);
}
```
- The first part of `lines` are the rules
- We access this part with `lines[0]`
- The rules are two numbers separated by a pipe
- We create an iterator over `lines[0]` with `iter()`
- Inside `map()` we split every line when we find a pipe
- We store a rule as a tuple and then advance the iterator to the second number with `next()`
- The first time we call `next()` we just call the first element
- `next()` return an Option type so that needs to be unwrapped
- And then we parse every element to a number of type `usize`
- `parse()` return a Result type so that needs also to be unwrapped
- All the resulting tuples are collected in a vector with `collect()`
- The following will print
```
[(47, 53), (97, 13), (97, 61), (...), (47, 29), (75, 13), (53, 13)]
```
```rust
fn process_input(input: &str) {
  let lines: Vec<_> = input.lines().collect();
  let lines: Vec<_> = lines.split(|line| line.is_empty()).collect();
  let rules: Vec<(usize, usize)> = lines[0].iter().map(|line| {
    let mut split = line.split("|");
    (split.next().unwrap().parse::<usize>().unwrap(), split.next().unwrap().parse::<usize>().unwrap())
  }).collect();
  println!("{:?}", rules);
}
```
- We do the same for the updates
- The difference is that that numbers are separated by commas and that we want a list of numbers
- Every update is a vector of numbers of type usize and all the updates are collected in a vector, thus `Vec<Vec<usize>`
- We map every line in `lines` to a line that is splited when we find a comma
- Every string in an update is parsed and then collected into a vector
- The following will print
```
[[75, 47, 61, 53, 29], [97, 61, 53, 29, 13], [75, 29, 13], [75, 97, 47, 61, 53], [61, 13, 29], [97, 13, 75, 29, 47]]
```
```rust
fn process_input(input: &str) {
  let lines: Vec<_> = input.lines().collect();
  let lines: Vec<_> = lines.split(|line| line.is_empty()).collect();
  (...)
  let update: Vec<Vec<usize>> = lines[1].iter().map(|line| {
    line.split(",").map(|s| s.parse::<usize>().unwrap()).collect()
  }).collect();
  println!("{:?}", update);
}
```
