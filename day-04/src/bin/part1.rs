fn process_input(input: &str) {
  let mut grid: Vec<String> = Vec::new();

  for line in input.lines() {
    grid.push(line.to_string());
  }

  let rows = grid.len();
  let cols = grid[0].len();

  for r in 0..rows {
    for c in 0..cols {
      println!("{r}, {c} : {:?}", grid[r].as_bytes()[c] as char);
    }
  }
}

fn main() {
  let file = include_str!("test-input.txt");
  process_input(file);
}
