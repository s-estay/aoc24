# Setup
- `mkdir aoc24`
- `cd aoc24`
- `git init`
- `git touch .gitignore` and `**/target/`
- `cargo new day-01 --vcs none`
- `cd day-01`
- `mkdir src/bin`
- `mv src/main.rs src/bin/part1.rs`
- `cargo run --bin part1`

# Part1

## Test input
- Expected result: 11
- `touch src/bin/test-input1.txt`
```
3   4
4   3
2   5
1   3
3   9
3   3
```

## Step by step solution
- Initial `main`
```rust
fn main() {
  println!("hello");
}
```
- Read input
```rust
fn process_part1(input: &str) {
  println!("{}", input);
}
fn main() {
  let file = include_str!("test-input1.txt");
  process_part1(file);
}
```
- How does iterators work in Rust?
- The following code will print `1` and then `2`
```rust
fn main() {
  let a = [1,2,3];
  let mut iter = a.iter();
  let b = iter.next().unwrap();
  println!("{}", b);
  let b = iter.next().unwrap();
  println!("{}", b);
}
```
- Iterate over every line
- `lines()` *returns an iterator over the lines of this reader*
```rust
fn process_part1(input: &str) {
  for line in input.lines() {
    println!("{}", line);
  }
}
fn main() {
  let file = include_str!("test-input1.txt");
  process_part1(file);
}
```
- Split `line` between left & right
- Take a look in the standard library for the string module [rust std::str](https://doc.rust-lang.org/std/str/)
- `SplitWhitespace` is *an iterator over the non-whitespace substrings of a string, separated by any amount of whitespace*
- The following will print: 3 4 4 3 2 5 1 3 3 9 3 3
- Notice that the numbers are displayed as "left row 1" "right row 1" "left row 2" "right row 2" and so on
```rust
fn process_part1(input: &str) {
  for line in input.lines() {
    let numbers = line.split_whitespace();
    for number in numbers {
      print!("{} ", number);
    }
  }
}
fn main() {
  let file = include_str!("test-input1.txt");
  process_part1(file);
}
```
- Create two mutable vectors to save the left/right numbers
- Save the numbers to the vectors with `push`
- Advance to the next number with iterator `next`
- `items` needs to be mutable to allow `next` to move forward
- Since `next` return an `Option<T>`, use `dbg!` to see if the vectors are correct
```rust
fn process_part1(input: &str) {
  let mut left = vec![];
  let mut right = vec![];
  for line in input.lines() {
    let mut numbers = line.split_whitespace();
    left.push(numbers.next());
    right.push(numbers.next());
  }
  dbg!(left, right);
}
fn main() {
  let file = include_str!("test-input1.txt");
  process_part1(file);
}
```
- Before we continue we need to take care of the result of type `Option<T>` that `next()` is returning
- The `Option<T>` type could be something (valid value) or could be nothing (more or less like returning `null`)
- In the case of `next`, it will be something until the iteration is finished (`None`)
- We will use `match` to manage the variants that `Option` might return: `Some` and `None`
- It's important to give the type annotation `&str` to the variables that store the result of `match`
- We will return an empty string when the iterator returns `None`
- Otherwise, we will return the inner value of `Some`
- We have two `match`, one for the left number, and one for the right number
- Since `match` is calling `next` we will advance the iterator one position everytime `match` is called
- After advancing the iterator, we will save the value to the corresponding vector
```rust
fn process_part1(input: &str) {
  let mut left = vec![];
  let mut right = vec![];
  for line in input.lines() {
    let mut numbers = line.split_whitespace();
    let ll: &str = match numbers.next() {
      None => "",
      Some(s) => s,
    };
    left.push(ll);
    let rr: &str = match numbers.next() {
      None => "",
      Some(s) => s,
    };
    right.push(rr);
  }
  dbg!(left, right);
}
fn main() {
  let file = include_str!("test-input1.txt");
  process_part1(file);
}
```
- At this point one might argue that we are better off using `unwrap` instead of `match` to deal with `Option<T>`
- Specially when we know that the AoC's inputs are not something unexpected that might return an invalid value
- Remember that `unwrap` will panic if the value is invalid
- But since I'm learning Rust, I want to handle all the cases explicitly
- For the curious out there, this is the same as above, but using `unwrap`
```rust
fn process_part1(input: &str) {
  let mut left = vec![];
  let mut right = vec![];
  for line in input.lines() {
    let mut numbers = line.split_whitespace();
    left.push(numbers.next().unwrap());
    right.push(numbers.next().unwrap());
  }
  dbg!(left, right);
}
fn main() {
  let file = include_str!("test-input1.txt");
  process_part1(file);
}
```
- Now we need to *pair up the smallest number in the left list with the smallest number in the right list, then the second-smallest left number with the second-smallest right number, and so on*
- This means sort the vectors using `sort` and convert the strings to integers using `parse`
- Since `parse` might fail, it return a `Result`, which have variants `Ok` and `Err`
- Just like `Option`, we will use `match` to manage the variants that `Result` might return
- We will go to the next iteration with `continue` if `parse` fails
- Otherwise, we will return the inner value of `Ok`
- The underscore in `Err(_)` will catch all the posible `Err` values
- I think both `match` for each left/right values can be done in one go with `unwrap`
```rust
fn process_part1(input: &str) {
  let mut left = vec![];
  let mut right = vec![];
  for line in input.lines() {
    let mut numbers = line.split_whitespace();
    let ll: &str = match numbers.next() {
      None => "",
      Some(s) => s,
    };
    let ll: i32 = match ll.parse() {
      Err(_) => continue,
      Ok(n) => n,
    };
    left.push(ll);
    let rr: &str = match numbers.next() {
      None => "",
      Some(s) => s,
    };
    let rr: i32 = match rr.parse() {
      Err(_) => continue,
      Ok(n) => n,
    };
    right.push(rr);
  }
  left.sort();
  right.sort();
  dbg!(left, right);
}
fn main() {
  let file = include_str!("test-input1.txt");
  process_part1(file);
}
```
- Finally we need to *figure out how far apart the two numbers are* and *find the total distance between the left list and the right list*
- The idea here is to (1) iterate through both vectors at the same time using `zip`
- Then (2) calculate the distance between the numbers using the absolute value `abs`
- And (3) calculate the total distance with `sum`
- The following will print (1,3) (2,3) (3,3) (3,4) (3,5) (4,9) // these are called `tuples`
```rust
fn process_part1(input: &str) {
  //...
  left.sort();
  right.sort();
  for i in left.iter().zip(right.iter()) {
    println!("{:?}", i);
  }
  dbg!(left, right);
}
fn main() {
  let file = include_str!("test-input1.txt");
  process_part1(file);
}
```
- Create a new mutable vector to store the distances
- `zip` returns a new iterator that will iterate over two other iterators (`left.iter()` and `right.iter()`)
- To access the elements of the tuple created by `zip` we do `i.0` and `i.1`
- Using the `zip` iterator we loop through the tuples and `push` the distance between the numbers to the vector
- Finally, we create another iterator `distances.iter()` to be able to add its elements together
- Again, I think it is possible to do the iterations in one go but I don't know how to do that yet
```rust
fn process_part1(input: &str) {
  //...
  left.sort();
  right.sort();
  let mut distances = vec![];
  for i in left.iter().zip(right.iter()) {
    distances.push((i.0 - i.1).abs());
  }
  let sp1: i32 = distances.iter().sum();
  println!("Solution part 1: {}", sp1);
}
fn main() {
  let file = include_str!("test-input1.txt");
  process_part1(file);
}
```
- Now get the "real" input and run your solution on that
- Hopefully the numbers will be similar enough for your solution to work
- `touch src/bin/input1.txt`
```rust
fn main() {
  let file = include_str!("input1.txt");
  process_part1(file);
}
```
# Part 2

## Setup
- `cp src/bin/part1.rs src/bin/part2.rs`

## Test input
- Expected result: 31
- Same test input as in part 1
- `cp src/bin/test-input1.txt src/bin/test-input2.txt`

## Step by step solution
- In part 2 we need to *calculate a total similarity score by adding up each number in the left list after multiplying it by the number of times that number appears in the right list*
- We can keep the code of part 1 until where we created the left/right vectors
- The vectors don't need to be sorted
- We start with the following code
```rust
fn process_part1(input: &str) {
  let mut left = vec![];
  let mut right = vec![];
  for line in input.lines() {
    let mut numbers = line.split_whitespace();
    let ll: &str = match numbers.next() {
      None => "",
      Some(s) => s,
    };
    let ll: i32 = match ll.parse() {
      Err(_) => continue,
      Ok(n) => n,
    };
    left.push(ll);
    let rr: &str = match numbers.next() {
      None => "",
      Some(s) => s,
    };
    let rr: i32 = match rr.parse() {
      Err(_) => continue,
      Ok(n) => n,
    };
    right.push(rr);
  }
  dbg!(left, right);
}
fn main() {
  let file = include_str!("test-input2.txt");
  process_part1(file);
}
```
- We start with an iterator for the left vector
- And for each number on left see how many times that number is repeated in the right vector
- The [trait iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html) show us all the stuff iterators can do
- To count the elements in a vector with some value without looping we can do the following
- Use `filter` and then `count` it
```rust
println!("count 3: {}", left.iter().filter(|&n| *n == 3).count());
```
- Now we can loop through the left vector and `filter` & `count` the right one
- The following will return 3 1 0 0 3 3
```rust
for i in left.iter() {
  println!("{}", right.iter().filter(|&n| *n == *i).count());
}
```
- Now we create a new vector to store the result
- Every value stored in the new vector is then multiplied by index `i` of the left vector
- Since `i` is of type `&u32`, we need to derefence it with `*i`
- Moreover, `count` results a `usize` so we need to cast `i` with `as usize`
- The final step is to add all the elements in the vector using `sum`
- `sum` requires to specify the type of the elements been added together using the **turbofish** syntax
```rust
fn process_part2(input: &str) {
  // ...
  let mut similarity_score = vec![];
  for i in left.iter() {
    similarity_score.push(right.iter().filter(|&n| *n == *i).count() * (*i as usize));
  }
  println!("Solution part 2: {:?}", similarity_score.iter().sum::<usize>());
}
fn main() {
  let file = include_str!("test-input2.txt");
  process_part2(file);
}
```
