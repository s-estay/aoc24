fn process_input(input: &str) {
  let mut grid: Vec<String> = Vec::new();

  for line in input.lines() {
    grid.push(line.to_string());
  }

  let rows = grid.len();
  let cols = grid[0].len();

  let xmas = "XMAS";

  //                                       N       NE      E     SE      S      SW      W        NW
  let directions: [(isize, isize); 8] = [(-1,0), (-1,1), (0,1), (1,1), (1,0), (1,-1), (0,-1), (-1,-1)];
  //let right: [(isize, isize); 1] = [(0,1)];

  let mut count = 0;

  for r in 0..rows {
    for c in 0..cols {
      for &(dx, dy) in &directions {
        if xmas.chars().enumerate().all(|(i, ch)| {
          let nx = r as isize + i as isize * dx;
          let ny = c as isize + i as isize * dy;

          nx >= 0 &&
          ny >= 0 &&
          (nx as usize) < cols &&
          (ny as usize) < rows &&
          grid[nx as usize].as_bytes()[ny as usize] as char == ch
        }) {
          count += 1;
        }
      }
    }
  }

  println!("{count}");
}

fn main() {
  let file = include_str!("input.txt");
  process_input(file);
}
