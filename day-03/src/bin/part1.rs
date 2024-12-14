use regex::Regex;

fn process_input(input: &str) {
  let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
  let sum: usize = re.captures_iter(&input).map(|group| {
    group[1].parse::<usize>().unwrap() * group[2].parse::<usize>().unwrap()
  }).sum();
  println!("{}", sum);
}

fn main() {
  let file = include_str!("input.txt");
  process_input(file);
}
