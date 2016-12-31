pub fn descending_order(x: u64) -> u64 {
  let mut number: Vec<_> = x.to_string().chars().collect();
  number.sort();
  number.reverse();
  number.into_iter().collect::<String>().parse::<u64>().unwrap()
}
