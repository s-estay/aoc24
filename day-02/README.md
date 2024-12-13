# Part 1

## Test input
- Expected result: 2
- Lines 1 and 6 are *safe*
```
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
```

## Step by step solution
- Read test input
```rust
fn process_part1(input: &str) {
  println!("{}", input);
}
fn main() {
  let file = include_str!("test-input.txt");
  process_part1(file);
}
```
- Create a counter that counts how many times a condition has returned `true`
- To do this we create an iterator that iterates over every lines with `lines`
- Then we filter that iterator with `filter`
- `filter` is part of the [iterator straits](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
- For test puposes, the filter´s clousure will always return `true`
- After that we will count all the filtered elements that return `true` with `count`
- `count` is also an iterator traits
- The following will return 6
```rust
fn process_part1(input: &str) {
  let count = input.lines().filter(|_line| {true}).count();
  println!("{}", count);
}
fn main() {
  let file = include_str!("test-input.txt");
  process_part1(file);
}
```
- Lets create a vector that store the parsed numbers of a line
- The numbers are separated by white spaces so we will split them with `split_whitespaces`
- We use the `map` part of `filter_map` to `parse` the numbers from strings to integers
- In simple words, `map` will take an iterator of type A and return another iterator of type B
- `map` is lazy, it is best used when you're already working with other iterators
- The `filter` part of `filter_map` allow us to check if `parse` is returning a valid number, thus avoiding `unwrap`
- If we for example pass a non numerical value, `filter_map` will just skip it instead of panicking
- Alternatively, we can write `map(|s| s.parse::<usize>().unwrap()).collect()`
- `parse` requires us to specify the type of the elements been parsed using the **turbofish** syntax `::<>`
- Since `count` returns a `usize`, we will `parse::<usize>` and will make `numbers` a `Vec<usize>`
- Finally, `collect` is used to transform an iterator into a collection (in this case, a vector)
```rust
fn process_part1(input: &str) {
  let count = input.lines().filter(|line| {
    let numbers: Vec<usize> = line.split_whitespace().filter_map(|s| s.parse::<usize>().ok()).collect();
    dbg!(numbers);
    true
  }).count();
  println!("{}", count);
}
fn main() {
  let file = include_str!("test-input.txt");
  process_part1(file);
}
```
- One way of going about solving this problem is to group the numbers in groups of 3 using `window`
- This will allow us to calculate the distance between the elements and to see in what order are they sorted
-  We can move a `window` using `next`
- The first line is 7 6 4 2 1
- The following will print [7,6,4] [6,4,2] and [4,2,1]
```rust
fn process_part1(input: &str) {
  let count = input.lines().filter(|line| {
    let numbers: Vec<usize> = line.split_whitespace().filter_map(|s| s.parse::<usize>().ok()).collect();
    let mut iter = numbers.windows(3);
    println!("{:?}", iter.next().unwrap());
    println!("{:?}", iter.next().unwrap());
    println!("{:?}", iter.next().unwrap());
    true
  }).count();
  println!("{}", count);
}
```
- We are still returning `true` so now it's time to evaluate the numbers in each window
- Use `all` to test every item of the iterator, ie every window for each vector
- First we check that the distances between the adjacent elements are between 1 and 3
- And then we check if the items are in ascending or descending order
- I think this solution is rather extensive but I'm happy with the result
```rust
fn process_part1(input: &str) {
  let count = input.lines().filter(|line| {
    let numbers: Vec<usize> = line.split_whitespace().filter_map(|s| s.parse::<usize>().ok()).collect();
    numbers.windows(3).all(|window| {
      (window[0].abs_diff(window[1]) > 0 &&
      window[0].abs_diff(window[1]) < 4 &&
      window[1].abs_diff(window[2]) > 0 &&
      window[1].abs_diff(window[2]) < 4) &&
      ((window[0] < window[1] && window[1] < window[2]) ||
      (window[0] > window[1] && window[1] > window[2]))
    })
  }).count();
  println!("{}", count);
}
fn main() {
  let file = include_str!("test-input.txt");
  process_part1(file);
}
```

## Part 2 (not done)

## Test input
- Expected result: 4
- From part 1 we got that reports 1 and 6 are *safe*
- But now we got new rules
- Report 4 is *safe* when we remove the second item **3**
- Report 5 is *safe* when we remove the third item **4**
```
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 (3) 2 4 5
8 6 (4) 4 1
1 3 6 7 9
```

## Step by step solution
- Check only unsafe reports
- Feed the reports to function `is_safe` that returns a `bool`
- The following will print *"Report [7, 6, 4, 2, 1] is safe"* and *"Report [1, 3, 6, 7, 9] is safe"*
```rust
fn is_safe(report: Vec<usize>) -> bool {
  report.windows(3).all(|window| {
    (window[0].abs_diff(window[1]) > 0 &&
    window[0].abs_diff(window[1]) < 4 &&
    window[1].abs_diff(window[2]) > 0 &&
    window[1].abs_diff(window[2]) < 4) &&
    ((window[0] < window[1] && window[1] < window[2]) ||
    (window[0] > window[1] && window[1] > window[2]))
  })
}
fn process_part2(input: &str) {
  for line in input.lines() {
    let report: Vec<usize> = line.split_whitespace().filter_map(|s| s.parse::<usize>().ok()).collect();
    if is_safe(report.clone()) { println!("Report {:?} is safe", report); }
    else { continue; }
  }
}
fn main() {
  let file = include_str!("test-input.txt");
  process_part2(file);
}
```
- Apparently the technique used by many was brute force, but I wasn't able to figure it out how to do that
