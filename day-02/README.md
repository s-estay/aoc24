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
- Apparently the technique used by many was brute force, so lets do that
- Functions `main` and `is_safe` stays the same
- Create a vector of vectors containing a copy of a report and all the variations of it when deleting one element
- We create as many new reports as numbers inside a report are (for the test input this is 5 new reports)
- We access every new report (sub vector) by mapping within a range
- We define a range using `(start .. end)`
- Every new report is a clone of a report where we have removed an item
- Don't forget to add the original report to `reports_vec` (vector of vectors)
```rust
fn is_safe(report: Vec<usize>) -> bool {
  // ...
}
fn process_part2(input: &str) {
  for line in input.lines() {
    let report: Vec<usize> = line.split_whitespace().filter_map(|s| s.parse::<usize>().ok()).collect();
    let mut reports_vec: Vec<Vec<usize>> = (0..report.len()).map(|i| {
      let mut new_report = report.clone();
      new_report.remove(i);
      new_report
    }).collect();
    reports_vec.push(report);
  }
}
fn main() {
  // ...
}
```
- Now we can loop through every new report and call `is_safe`
- For the first report [7, 6, 4, 2, 1] we will print the following
```
Report [6, 4, 2, 1] is safe
Report [7, 4, 2, 1] is safe
Report [7, 6, 2, 1] is not safe
Report [7, 6, 4, 1] is safe
Report [7, 6, 4, 2] is safe
Report [7, 6, 4, 2, 1] is safe
```
```rust
fn is_safe(report: Vec<usize>) -> bool {
  // ...
}
fn process_part2(input: &str) {
  for line in input.lines() {
    let report: Vec<usize> = line.split_whitespace().filter_map(|s| s.parse::<usize>().ok()).collect();
    let mut reports_vec: Vec<Vec<usize>> = (0..report.len()).map(|i| {
      let mut new_report = report.clone();
      new_report.remove(i);
      new_report
    }).collect();
    reports_vec.push(report);
    for i in reports_vec {
      if is_safe(i.clone()) { println!("Report {:?} is safe", i); }
      else { println!("Report {:?} is not safe", i); }
    }
  }
}
fn main() {
  // ...
}
```
- Now lets count the safe reports
- The following prints 5 0 0 2 2 4 which is correct but we need to add them together
```rust
let count = reports_vec.iter().filter(|report| is_safe(*report)).count();
println!("{}", count);
```
- Notice that the argument of `is_safe` is now a reference
- All the values we got before are now stored in a vector defined outside the main loop
- `sum` is used to add them together
- The type that `sum` is returning is defined using the `::<>` notation
```rust
fn is_safe(report: &Vec<usize>) -> bool {
  // ...
}
fn process_part2(input: &str) {
  let mut sp2: Vec<usize> = vec![];
  for line in input.lines() {
    let report: Vec<usize> = line.split_whitespace().filter_map(|s| s.parse::<usize>().ok()).collect();
    let mut reports_vec: Vec<Vec<usize>> = (0..report.len()).map(|i| {
      let mut new_report = report.clone();
      new_report.remove(i);
      new_report
    }).collect();
    reports_vec.push(report);
    sp2.push(reports_vec.iter().filter(|report| is_safe(*report)).count());
  }
  println!("{:?}", sp2.iter().sum::<usize>());
}
fn main() {
  // ...
}
```
- Unfortunately I did't get the correct result

## Update
- I was out on a long walk with the dogs when it hit me
- If a report is unsafe, remove one element and test it
- If a report becomes safe after removing one element, don't continue removing elements and testing
- Additionally, a safe report doesn't need to go through the process of removing and testing
- Create a vector of vectors `all_reports` outside the loop
- If the report is safe, push a copy of it to `all_reports`
- If the report is unsafe, clone the report a to new report
- Inside a loop in range [0, vector length] remove a number and test `new_report`
- If `new_report` it's safe, `break` out of the loop and push a copy to `all_reports`
- After all that is done, count the number of sub vectors stored in `all_reports` using an iterator
- This is still brute force but whatever
```rust
fn is_safe(report: &Vec<usize>) -> bool {
  // ...
}
fn process_part2(input: &str) {
  let mut all_reports = Vec::new();
  for line in input.lines() {
    let report: Vec<usize> = line.split_whitespace().filter_map(|s| s.parse::<usize>().ok()).collect();
    if is_safe(&report) { all_reports.push(report.clone()); }
    else {
      for bad_level in 0..report.len() {
        let mut new_report = report.clone();
        new_report.remove(bad_level);
        if is_safe(&new_report) {
          all_reports.push(new_report.clone());
          break;
        }
      }
    }
  }
  println!("{:?}", all_reports.iter().count());
}
fn main() {
  // ...
}
```
