use proconio::{input as I, fastout as F};

#[F]
fn main() {
  I! {t: u32}
  for _ in 0..t {
    I! {
      n: usize,
      wp: [(i64, i64); n]
    }
    let mut wpd = wp.iter().map(|&(w, p)| (w, p, p + w)).collect::<Vec<(_, _, _)>>();
    wpd.sort_by(|a, b| b.2.cmp(&a.2));
    let mut sw = wpd.iter().fold(0, |s, &(w, _, _)| s + w);
    let mut sp = 0;
    let mut c = n;
    dbg!(sw, sp, c);
    for (w, p, _) in wpd {
      sw -= w;
      sp += p;
      c -= 1;
      dbg!(sw, sp, c);
      if sw <= sp {break}
    }
    println!("{c}");
  }
}
