fn process_input(input: &str) {
  println!("{}", input);
}

fn main() {
  let file = include_str!("test-input.txt");
  process_input(file);
}
