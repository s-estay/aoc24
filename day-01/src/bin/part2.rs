fn process_part2(input: &str) {
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

  let mut similarity_score = vec![];
  for i in left.iter() {
    similarity_score.push(right.iter().filter(|&n| *n == *i).count() * (*i as usize));
  }

  println!("Solution part 2: {:?}", similarity_score.iter().sum::<usize>());
}

fn main() {
  let file = include_str!("input2.txt");
  process_part2(file);
}
