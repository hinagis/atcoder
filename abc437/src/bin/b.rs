use proconio::input as I;

fn main() {
    I! {
        h: usize,
        w: usize,
        n: usize,
        a: [[u32; w]; h]
    }
    let mut c = vec![0; h];
    for _ in 0..n {
      I! {b: u32}
      for i in 0..h {
        for j in 0..w {
          if b == a[i][j] {
            c[i] += 1;
          }
        }
      }
    }
    println!("{}", c.iter().fold(0, |m, &c| m.max(c)));
}
