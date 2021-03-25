fn main() {
  const LIMIT: usize = std::usize::MAX;

  let mut cnt: i32 = 0;
  let mut r: Vec<bool> = vec![true; LIMIT];

  for i in 2..r.len() {
    if !r[i] {
      continue;
    }
    cnt += 1;
    if cnt == 10_000_000 {
      println!("{} is completed!", i);
      break;
    }

    for j in ((2 * i)..r.len()).step_by(i) {
      r[j] = false;
    }
  }
}
