use itertools::Itertools;
use proconio::input as I;

fn main() {
    I! {
        n: usize,
        m: usize,
    }
    let mut h = std::collections::HashSet::new();
    let mut s = 0;
    for _ in 0..m {
      I! {
        r: usize,
        c: usize
      }
      if r > 0 {
        if c > 0 && h.contains(&(r - 1, c - 1)) {continue}
        if h.contains(&(r - 1, c)) {continue}
        if h.contains(&(r - 1, c + 1)) {continue}
      }
      if c > 0 {
        if h.contains(&(r, c - 1)) {continue}
        if h.contains(&(r + 1, c - 1)) {continue}
      }
      if h.contains(&(r, c)) {continue}
      if h.contains(&(r, c + 1)) {continue}
      if h.contains(&(r + 1, c)) {continue}
      if h.contains(&(r + 1, c + 1)) {continue}
      s += 1;
      h.insert((r, c));
    }
    println!("{s}");
}
