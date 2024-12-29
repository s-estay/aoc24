fn process_input(input: &str) {
  let lines: Vec<_> = input.lines().collect();
  let lines: Vec<_> = lines.split(|line| line.is_empty()).collect();

  let _rules: Vec<(usize, usize)> = lines[0].iter().map(|line| {
    let mut split = line.split("|");
    (split.next().unwrap().parse::<usize>().unwrap(), split.next().unwrap().parse::<usize>().unwrap())
  }).collect();

  let update: Vec<Vec<usize>> = lines[1].iter().map(|line| {
    line.split(",").map(|s| s.parse::<usize>().unwrap()).collect()
  }).collect();

  println!("{:?}", update);
}

fn main() {
  let file = include_str!("test-input.txt");
  process_input(file);
}
