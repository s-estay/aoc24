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
  let mut sp2: Vec<usize> = vec![];
  for line in input.lines() {
    let report: Vec<usize> = line.split_whitespace().filter_map(|s| s.parse::<usize>().ok()).collect();

    let mut reports_vec: Vec<Vec<usize>> = (0..report.len()).map(|i| {
      let mut new_report = report.clone();
      new_report.remove(i);
      new_report
    }).collect();

    reports_vec.push(report);

    //for i in reports_vec {
    //  if is_safe(*i) { println!("Report {:?} is safe", i); }
    //  else { println!("Report {:?} is not safe", i); }
    //}

    sp2.push(reports_vec.iter().filter(|report| is_safe(*report)).count());
  }
  println!("{:?}", sp2.iter().sum::<usize>());
}

fn main() {
  let file = include_str!("input.txt");
  process_part2(file);
}
