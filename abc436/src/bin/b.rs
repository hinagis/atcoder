use itertools::Itertools;
use proconio::input as I;

fn main() {
    I! {
        n: usize,
    }
    let mut b = vec![vec![0; n]; n];
    let mut i = 0;
    let mut j = (n - 1) / 2;
    let mut c = 0;
    while c < n * n {
      c += 1;
      b[i][j] = c;
      let r = (i + n - 1) % n;
      let c = (j + 1) % n;
      (i, j) = if b[r][c] > 0 {
        ((i + 1) % n, j)
      } else {
        (r, c)
      };
    }
    println!("{}", b.iter().map(|s| s.iter().join(" ")).join("\n"));
}
