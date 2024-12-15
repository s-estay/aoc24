use regex::Regex;

fn process_input(input: &str) {
  let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
  let re_do = Regex::new(r"do\(\)").unwrap();
  let re_dont = Regex::new(r"don\'t\(\)").unwrap();

  let mut dos: Vec<usize> = Vec::new();
  let mut donts: Vec<usize> = Vec::new();

  for capture in re_do.captures_iter(&input) {
    dos.push(capture.get(0).unwrap().start());
  }

  for capture in re_dont.captures_iter(&input) {
    donts.push(capture.get(0).unwrap().start());
  }

  let sum: usize = re
    .captures_iter(&input)
    .filter(|capture| {
      let index = capture.get(0).unwrap().start();
      let last_do = dos.iter().filter(|&&i| i < index).max();
      let last_dont = donts.iter().filter(|&&i| i < index).max();

      if last_do.is_some() && last_dont.is_some() { last_do > last_dont }
      else if last_do.is_some() && last_dont.is_none() { return true; }
      else if last_do.is_none() && last_dont.is_some() { return false; }
      else { return true; }
    })
    .map(|group| {
      group[1].parse::<usize>().unwrap() * group[2].parse::<usize>().unwrap()
    })
    .sum();
  println!("{}", sum);
}

fn main() {
  let file = include_str!("input.txt");
  process_input(file);
}
