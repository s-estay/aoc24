fn is_safe(report: Vec<usize>) -> bool {
  report.windows(3).all(|window| {
    (window[0].abs_diff(window[1]) > 0 &&
    window[0].abs_diff(window[1]) < 4 &&
    window[1].abs_diff(window[2]) > 0 &&
    window[1].abs_diff(window[2]) < 4) &&
    ((window[0] < window[1] && window[1] < window[2]) ||
    (window[0] > window[1] && window[1] > window[2]))
  })
}

fn process_part2(input: &str) {
  for line in input.lines() {
    let report: Vec<usize> = line.split_whitespace().filter_map(|s| s.parse::<usize>().ok()).collect();
    if is_safe(report.clone()) { println!("Report {:?} is safe", report.clone()); }
    else { continue; }
  }
}

fn main() {
  let file = include_str!("test-input.txt");
  process_part2(file);
}
