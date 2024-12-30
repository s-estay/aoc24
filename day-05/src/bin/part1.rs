use itertools::Itertools;

fn process_input(input: &str) {
  let lines: Vec<_> = input.lines().collect();
  let lines: Vec<_> = lines.split(|line| line.is_empty()).collect();

  let rules: Vec<(usize, usize)> = lines[0].iter().map(|line| {
    let mut split = line.split("|");
    (split.next().unwrap().parse::<usize>().unwrap(), split.next().unwrap().parse::<usize>().unwrap())
  }).collect();

  let updates: Vec<Vec<usize>> = lines[1].iter().map(|line| {
    line.split(",").map(|s| s.parse::<usize>().unwrap()).collect()
  }).collect();

  let sum: usize = updates.iter().filter(|update| {
    !update.iter().combinations(2).map(|v| (v[0], v[1])).any(|(&x, &y)| rules.iter().any(|r| r.1 == x && r.0 == y))
    }).map(|update| update[update.len()/2]).sum();

  println!("{:?}", sum);
}

fn main() {
  let file = include_str!("input.txt");
  process_input(file);
}
