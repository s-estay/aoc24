use regex::Regex;

fn process_input(input: &str) {
  let xmas = Regex::new(r"XMAS").unwrap();
  let samx = Regex::new(r"SAMX").unwrap();

  let sum: usize = xmas.find_iter(&input).count() + samx.find_iter(&input).count();
  println!("{}", sum);
}

fn main() {
  let file = include_str!("test-input.txt");
  process_input(file);
}
