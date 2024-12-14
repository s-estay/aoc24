fn is_safe(report: &Vec<usize>) -> bool {
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
  let mut all_reports = Vec::new();
  for line in input.lines() {
    let report: Vec<usize> = line.split_whitespace().filter_map(|s| s.parse::<usize>().ok()).collect();

    if is_safe(&report) { all_reports.push(report.clone()); }
    else {
      for bad_level in 0..report.len() {
        let mut new_report = report.clone();
        new_report.remove(bad_level);
        if is_safe(&new_report) {
          all_reports.push(new_report.clone());
          break;
        }
      }
    }
  }
  println!("{:?}", all_reports.iter().count());
}

fn main() {
  let file = include_str!("input.txt");
  process_part2(file);
}
