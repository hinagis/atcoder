use proconio::input as I;
const M: usize = 998244353;

fn main() {
  I! {
    n: usize,
    m: usize,
    mut a: [usize; n],
    mut b: [usize; m]
  }
  a.sort();
  b.sort();
  let mut c = vec![0; m + 1];
  for j in 0..m {
    c[j + 1] += (c[j] + b[j]) % M;
  }
  let mut s = 0;
  let mut j = 0;
  for i in 0..n {
    while j < m && a[i] > b[j] {
      j += 1;
    }
    s += c[m] + M - c[j];
    s %= M;
    s += M;
    s -= (a[i] * (m - j)) % M;
    s %= M;

    s += a[i] * j;
    s %= M;
    s += M;
    s -= c[j];
    s %= M;
  }
  println!("{s}");
}
