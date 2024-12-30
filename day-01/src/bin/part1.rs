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

  let mut distances = vec![];
  for i in left.iter().zip(right.iter()) {
    distances.push((i.0 - i.1).abs());
  }

  let sp1: i32 = distances.iter().sum();
  println!("Solution part 1: {}", sp1);
}

fn main() {
  let file = include_str!("input.txt");
  process_part1(file);
}
