fn fin_x(grid: &Vec<String>) -> Vec<(usize, usize)> {
  let mut position: Vec<(usize, usize)> = Vec::new();

  for r in 0..grid.len() {
    for c in 0..grid[0].len() {
      if grid[r].as_bytes()[c] as char == 'X' {
        position.push((r, c));
      }
      else {
        continue;
      }
    }
  }
  position
}

fn process_input(input: &str) {
  let mut grid: Vec<String> = Vec::new();

  for line in input.lines() {
    grid.push(line.to_string());
  }

  println!("{:?}", fin_x(&grid));
}

fn main() {
  let file = include_str!("test-input.txt");
  process_input(file);
}
