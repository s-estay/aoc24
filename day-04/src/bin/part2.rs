fn process_input(input: &str) {
  let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

  let rows = grid.len();
  let cols = grid[0].len();

  let mut count = 0;

  for r in 1..rows-1 {
    for c in 1..cols-1 {
      let cc = grid[r][c];     // center
      let tl = grid[r-1][c-1]; // top left
      let tr = grid[r-1][c+1]; // top right
      let bl = grid[r+1][c-1]; // bottom left
      let br = grid[r+1][c+1]; // bottom right

      if cc == 'A' {
       let d0 = (tl == 'M' && br == 'S') || (tl == 'S' && br == 'M');
       let d1 = (tr == 'M' && bl == 'S') || (tr == 'S' && bl == 'M');

       if d0 && d1 { count += 1; }
      }
    }
  }
  println!("{count}");
}

fn main() {
  let file = include_str!("input.txt");
  process_input(file);
}
