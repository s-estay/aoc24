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
  let file = include_str!("input.txt");
  process_part1(file);
}
