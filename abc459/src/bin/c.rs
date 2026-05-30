use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        n: usize,
        q: usize,
    }
    let mut t = vec![0; q];
    let mut c = vec![0; n];
    let mut b = 0;
    for _ in 0..q {
        I! {
            q: u8,
            x: usize
        }
        if q == 1 {
            let x = x - 1;
            c[x] += 1;
            t[c[x]] += 1;
            if t[c[x]] >= n {
                b += 1;
            }
        } else {
            println!("{}", t.get(x + b).unwrap_or(&0));
        }
    }
}
