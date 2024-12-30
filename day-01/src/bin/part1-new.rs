fn process_input(input: &str) {
  let (mut left, mut right):(Vec<usize>, Vec<usize>) = input.lines().map(|line| {
    let mut split = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
    (split.next().unwrap(), split.next().unwrap())
  }).unzip();
  left.sort();
  right.sort();

  let sum: usize = left.iter().zip(right.iter()).map(|(l, r)| l.abs_diff(*r)).sum();
  println!("{:?}", sum);
}
fn main() {
  let file = include_str!("test-input.txt");
  process_input(file);
}
