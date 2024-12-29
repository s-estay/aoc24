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
  let updates: Vec<Vec<usize>> = lines[1].iter().map(|line| {
    line.split(",").map(|s| s.parse::<usize>().unwrap()).collect()
  }).collect();
  println!("{:?}", updates);
}
```
- Now we need to retrieve the middle value of an update
- To do this, we iterate through the updates with `iter()`
- And then locate the value with `map()`, where `update.len()/2` is the index of the middle value
- The following will print
```
[61, 53, 29, 47, 13, 75]
```
```rust
fn process_input(input: &str) {
  (...)
  let middle_value: Vec<usize> = updates.iter().map(|update| update[update.len()/2]).collect();
  println!("{:?}", middle_value);
}
```
- Now we add all the middle values like this
```rust
fn process_input(input: &str) {
  (...)
  let sum: usize = updates.iter().map(|update| update[update.len()/2]).sum();
  println!("{:?}", sum);
}
```
- But we are been asked to only add the middle value of the updates that follow the rules
- For this we will use a filter before `map()`
```rust
fn process_input(input: &str) {
  (...)
  let sum: usize = updates.iter().filter(|_update| true).map(|update| update[update.len()/2]).sum();
  println!("{:?}", sum);
}
```
- The idea is to compare all the combinationes of pair of two number of the updates against every rule
- To create all the combinations we will use the crate [itertools](https://crates.io/crates/itertools) and its method `combinations`
- The following will print
```
[[[75, 47], [75, 61], [75, 53], [75, 29], [47, 61], [47, 53], [47, 29], [61, 53], [61, 29], [53, 29]], (...)]
```
```rust
fn process_input(input: &str) {
  (...)
  let aa: Vec<Vec<_>> = updates.iter().map(|update| update.iter().combinations(2).collect()).collect();
  println!("{:?}", aa);
}
```
- Every pair of numbers can be stored as a tuple with `map()`
```
[[(75, 47), (75, 61), (75, 53), (...)]
```
```rust
fn process_input(input: &str) {
  (...)
  let aa: Vec<Vec<(&usize, &usize)>> = updates.iter().map(|update| update.iter().combinations(2).map(|v| (v[0], v[1])).collect()).collect();
  println!("{:?}", aa);
}
```
- Update **75,97,47,61,53** fails because it doesn't follow rule **97|75**
- The first number in update subsequence **75,97** is equal to the second number of rule **97|75**
- And the second number in update subsequence **75,97** is equal to the first number of rule **97|75**
- When this happens we can say that the update doesn't follow the rule
- But we want to keep the updates that do follow the rules, so we invert the condition of the filter with `!`
```rust
fn process_input(input: &str) {
  (...)
  let sum: usize = updates.iter().filter(|update| {
    !update.iter().combinations(2).map(|v| (v[0], v[1])).any(|(&x, &y)| rules.iter().any(|r| r.1 == x && r.0 == y))
    }).map(|update| update[update.len()/2]).sum();
  println!("{:?}", sum);
}
```
- Notice in the code above how Rust handles embedded interators
- We first iterate over the updates which are stored in a vector of vectors
- Inside `filter()` we iterate over every number in an update, creating tuples with `map()`
- `filter()` get its boolean expression from `any()` that iterates over the rules
